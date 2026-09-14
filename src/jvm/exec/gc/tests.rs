use super::*;

fn cycle(heap: &mut Heap) -> (RuntimeValue, rc::Weak<cell::RefCell<JVMObject>>, rc::Weak<cell::RefCell<Vec<i32>>>) {
    let object = rc::Rc::new(cell::RefCell::new(JVMObject {
        class: "java/lang/Object".to_string(), fields: HashMap::new(),
    }));
    let payload = rc::Rc::new(cell::RefCell::new(vec![42; 1024]));
    let value = RuntimeValue::Object(object.clone());
    object.borrow_mut().fields.insert("self".to_string(), value.clone());
    object.borrow_mut().fields.insert("payload".to_string(), RuntimeValue::IntArray(payload.clone()));
    heap.observe(&value);
    (value, rc::Rc::downgrade(&object), rc::Rc::downgrade(&payload))
}

#[test]
fn unreachable_cycle_and_payload_are_freed() {
    let mut heap = Heap::default();
    let (value, object, payload) = cycle(&mut heap);
    drop(value);
    assert!(object.upgrade().is_some()); // Reference counting alone leaks it.
    heap.collect(Vec::new());
    assert!(object.upgrade().is_none());
    assert!(payload.upgrade().is_none());
    assert!(heap.allocations.is_empty());
}

#[test]
fn all_runtime_roots_keep_cycles_alive_until_removed() {
    for kind in 0..6 {
        let jvm = create_runtime_const();
        let mut frame = Frame { locals: Vec::new(), stack: Vec::new() };
        let (value, object, payload) = cycle(&mut jvm.heap.borrow_mut());
        let guard = match kind {
            0 => { frame.locals.push(value); None },
            1 => { frame.stack.push(value); None },
            2 => {
                jvm.classes["java/lang/Object"].fields.borrow_mut().insert("root".to_string(), value);
                None
            },
            3 => { *jvm.pending_exception.borrow_mut() = Some(value); None },
            4 => { jvm.monitors.borrow_mut().push((value, 1)); None },
            _ => {
                let caller = Frame { locals: vec![value.clone()], stack: vec![value] };
                Some(CallRoots::new(&jvm.heap, &caller))
            },
        };
        jvm.heap.borrow_mut().allocated = ALLOCATION_BUDGET;
        collect_if_due(&jvm, &frame);
        assert!(object.upgrade().unwrap().borrow().fields.contains_key("self"));
        assert_eq!(payload.upgrade().unwrap().borrow()[0], 42);
        frame.locals.clear();
        frame.stack.clear();
        jvm.classes["java/lang/Object"].fields.borrow_mut().clear();
        jvm.pending_exception.borrow_mut().take();
        jvm.monitors.borrow_mut().clear();
        drop(guard);
        jvm.heap.borrow_mut().allocated = ALLOCATION_BUDGET;
        collect_if_due(&jvm, &frame);
        assert!(object.upgrade().is_none(), "root kind {}", kind);
        assert!(payload.upgrade().is_none());
    }
}

#[test]
fn object_array_cycle_is_traced_and_reclaimed() {
    let mut heap = Heap::default();
    let (object, weak_object, payload) = cycle(&mut heap);
    let array = rc::Rc::new(cell::RefCell::new(JVMReferenceArray {
        component_class: "java/lang/Object".to_string(), values: vec![object.clone()],
    }));
    let weak_array = rc::Rc::downgrade(&array);
    if let RuntimeValue::Object(value) = &object {
        value.borrow_mut().fields.insert("array".to_string(), RuntimeValue::ReferenceArray(array.clone()));
    }
    heap.observe(&RuntimeValue::ReferenceArray(array.clone()));
    drop(object);
    heap.collect(vec![RuntimeValue::ReferenceArray(array.clone())]);
    assert_eq!(array.borrow().values.len(), 1);
    assert!(payload.upgrade().is_some());
    drop(array);
    heap.collect(Vec::new());
    assert!(weak_array.upgrade().is_none());
    assert!(weak_object.upgrade().is_none());
    assert!(payload.upgrade().is_none());
}
