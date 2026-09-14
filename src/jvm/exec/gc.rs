//! Single-threaded tracing cycle collector layered over reference counting.
//! Weak registrations never keep otherwise unreachable allocations alive.
use super::*;
use std::collections::HashSet;

#[cfg(test)]
mod tests;

// Allocation pressure, not process RSS: includes exact primitive array payload
// capacities and an estimate for reference-bearing allocation overhead.
const ALLOCATION_BUDGET: usize = 16 * 1024 * 1024;

type Key = (u8, usize);
enum Allocation {
    Object(rc::Weak<cell::RefCell<JVMObject>>),
    Array(rc::Weak<cell::RefCell<JVMReferenceArray>>),
}
impl Allocation {
    fn upgrade(&self) -> Option<RuntimeValue> {
        match self {
            Self::Object(value) => value.upgrade().map(RuntimeValue::Object),
            Self::Array(value) => value.upgrade().map(RuntimeValue::ReferenceArray),
        }
    }
}

#[derive(Default)]
pub(super) struct Heap {
    allocations: HashMap<Key, Allocation>,
    allocated: usize,
    suspended: Vec<Vec<RuntimeValue>>,
}

fn key(value: &RuntimeValue) -> Option<Key> {
    match value {
        RuntimeValue::Object(value) => Some((0, rc::Rc::as_ptr(value) as usize)),
        RuntimeValue::ReferenceArray(value) => Some((1, rc::Rc::as_ptr(value) as usize)),
        _ => None,
    }
}

impl Heap {
    // Called for values exposed on the operand stack, including exceptions and
    // native return values. Replacing an expired weak entry handles address reuse.
    pub(super) fn observe(&mut self, value: &RuntimeValue) {
        if let Some(key) = key(value) {
            let fresh = self.allocations.get(&key).map_or(true, |entry| entry.upgrade().is_none());
            if fresh {
                let allocation = match value {
                    RuntimeValue::Object(value) => Allocation::Object(rc::Rc::downgrade(value)),
                    RuntimeValue::ReferenceArray(value) => Allocation::Array(rc::Rc::downgrade(value)),
                    _ => unreachable!(),
                };
                self.allocations.insert(key, allocation);
                self.allocated = self.allocated.saturating_add(128);
            }
        }
    }

    // Newly allocated arrays form trees, so this walk needs no cycle detection.
    pub(super) fn allocated_array(&mut self, value: &RuntimeValue) {
        self.observe(value);
        let bytes = match value {
            RuntimeValue::IntArray(v) => v.borrow().capacity() * 4,
            RuntimeValue::LongArray(v) => v.borrow().capacity() * 8,
            RuntimeValue::DoubleArray(v) => v.borrow().capacity() * 8,
            RuntimeValue::FloatArray(v) => v.borrow().capacity() * 4,
            RuntimeValue::ShortArray(v) => v.borrow().capacity() * 2,
            RuntimeValue::CharArray(v) => v.borrow().capacity() * 2,
            RuntimeValue::ByteArray(v) => v.borrow().values.capacity(),
            RuntimeValue::ReferenceArray(v) => {
                let array = v.borrow();
                for child in &array.values { self.allocated_array(child); }
                array.values.capacity() * std::mem::size_of::<RuntimeValue>()
            },
            _ => 0,
        };
        self.allocated = self.allocated.saturating_add(bytes);
    }

    pub(super) fn due(&self) -> bool { self.allocated >= ALLOCATION_BUDGET }

    pub(super) fn collect(&mut self, mut roots: Vec<RuntimeValue>) {
        for frame in &self.suspended { roots.extend(frame.iter().cloned()); }
        let mut live = HashSet::new();
        while let Some(value) = roots.pop() {
            if let Some(key) = key(&value) {
                if !live.insert(key) { continue; }
                match value {
                    RuntimeValue::Object(value) => roots.extend(value.borrow().fields.values().cloned()),
                    RuntimeValue::ReferenceArray(value) => roots.extend(value.borrow().values.iter().cloned()),
                    _ => unreachable!(),
                }
            }
        }
        // Hold all garbage until every outgoing edge has been removed. This
        // avoids recursive destruction of chains while breaking cyclic graphs.
        let dead: Vec<_> = self.allocations.iter()
            .filter(|(key, _)| !live.contains(key))
            .filter_map(|(_, value)| value.upgrade()).collect();
        for value in &dead {
            match value {
                RuntimeValue::Object(value) => value.borrow_mut().fields.clear(),
                RuntimeValue::ReferenceArray(value) => value.borrow_mut().values.clear(),
                _ => unreachable!(),
            }
        }
        debug!("GC: {} live graph nodes, {} dead graph nodes", live.len(), dead.len());
        drop(dead);
        self.allocations.retain(|_, value| value.upgrade().is_some());
        self.allocated = 0;
    }
}

impl Drop for Heap {
    fn drop(&mut self) {
        // Runtime teardown has no Java roots. Reclaim cycles below the budget
        // too, including when execute_class_file is called repeatedly in-process.
        self.suspended.clear();
        self.collect(Vec::new());
    }
}

// Call operands remain conservatively rooted while a callee executes. RAII
// removes snapshots on normal return and on every interpreter error path.
pub(super) struct CallRoots<'a>(&'a cell::RefCell<Heap>);
impl<'a> CallRoots<'a> {
    pub(super) fn new(heap: &'a cell::RefCell<Heap>, frame: &Frame) -> Self {
        heap.borrow_mut().suspended.push(frame.locals.iter().chain(&frame.stack).cloned().collect());
        Self(heap)
    }
}
impl Drop for CallRoots<'_> {
    fn drop(&mut self) { self.0.borrow_mut().suspended.pop(); }
}

pub(super) fn collect_if_due(jvm: &RuntimeConst, frame: &Frame) {
    if !jvm.heap.borrow().due() { return; }
    let mut roots: Vec<_> = frame.locals.iter().chain(&frame.stack).cloned().collect();
    for class in jvm.classes.values() { roots.extend(class.fields.borrow().values().cloned()); }
    roots.extend(jvm.pending_exception.borrow().iter().cloned());
    roots.extend(jvm.monitors.borrow().iter().map(|(value, _)| value.clone()));
    // Interned strings contain no outgoing managed references and are already
    // owned by the runtime's intern table.
    jvm.heap.borrow_mut().collect(roots);
}
