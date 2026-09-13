use super::*;

fn zeroed<T: Default + Clone>(count: usize) -> Result<rc::Rc<cell::RefCell<Vec<T>>>, String> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).map_err(|error| error.to_string())?;
    values.resize(count, T::default());
    Ok(rc::Rc::new(cell::RefCell::new(values)))
}

pub(super) fn allocate_multidimensional(descriptor: &str, counts: &[usize]) -> Result<RuntimeValue, String> {
    let component = descriptor.strip_prefix('[').ok_or("invalid array descriptor")?;
    let count = *counts.first().ok_or("missing array dimensions")?;
    if counts.len() > 1 && !component.starts_with('[') {
        return Err("too many array dimensions".to_string());
    }
    if component.starts_with('[') || (component.starts_with('L') && component.ends_with(';')) {
        let component_class = if component.starts_with('[') {
            component.to_string()
        } else {
            component[1..component.len() - 1].to_string()
        };
        let mut values = Vec::new();
        values.try_reserve_exact(count).map_err(|error| error.to_string())?;
        for _ in 0..count {
            // Allocate each child separately: cloning a shared array would alias rows.
            values.push(if counts.len() > 1 {
                allocate_multidimensional(component, &counts[1..])?
            } else {
                RuntimeValue::Null // Unallocated dimensions or reference elements.
            });
        }
        return Ok(RuntimeValue::ReferenceArray(rc::Rc::new(cell::RefCell::new(
            JVMReferenceArray { component_class, values }
        ))));
    }
    Ok(match component {
        "I" => RuntimeValue::IntArray(zeroed(count)?),
        "J" => RuntimeValue::LongArray(zeroed(count)?),
        "F" => RuntimeValue::FloatArray(zeroed(count)?),
        "D" => RuntimeValue::DoubleArray(zeroed(count)?),
        "C" => RuntimeValue::CharArray(zeroed(count)?),
        "S" => RuntimeValue::ShortArray(zeroed(count)?),
        "B" | "Z" => {
            let mut values = Vec::new();
            values.try_reserve_exact(count).map_err(|error| error.to_string())?;
            values.resize(count, 0);
            RuntimeValue::ByteArray(rc::Rc::new(cell::RefCell::new(JVMByteArray {
                is_boolean: component == "Z", values,
            })))
        },
        _ => return Err("unsupported array descriptor".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_are_independent() {
        let array = allocate_multidimensional("[[I", &[2, 3]).unwrap();
        let RuntimeValue::ReferenceArray(rows) = array else { panic!("expected rows") };
        let rows = rows.borrow();
        assert_eq!(rows.component_class, "[I");
        let RuntimeValue::IntArray(first) = &rows.values[0] else { panic!("expected int row") };
        let RuntimeValue::IntArray(second) = &rows.values[1] else { panic!("expected int row") };
        first.borrow_mut()[1] = 42;
        assert_eq!(*second.borrow(), vec![0, 0, 0]);
    }

    #[test]
    fn partial_dimensions_and_reference_leaves_are_null() {
        for descriptor in ["[[[I", "[Ljava/lang/String;"] {
            let array = allocate_multidimensional(descriptor, &[2]).unwrap();
            let RuntimeValue::ReferenceArray(array) = array else { panic!("expected references") };
            assert!(matches!(array.borrow().values.as_slice(), [RuntimeValue::Null, RuntimeValue::Null]));
        }
    }

    #[test]
    fn empty_dimension() {
        let array = allocate_multidimensional("[[I", &[0, 3]).unwrap();
        let RuntimeValue::ReferenceArray(array) = array else { panic!("expected references") };
        assert!(array.borrow().values.is_empty());
    }
}
