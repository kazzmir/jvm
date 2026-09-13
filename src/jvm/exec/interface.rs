use super::*;

fn implements(jvm: &RuntimeConst, class_name: &str, interface: &str, seen: &mut Vec<String>) -> Result<bool, String> {
    if class_name == interface { return Ok(true); }
    if seen.iter().any(|name| name == class_name) { return Ok(false); }
    seen.push(class_name.to_string());
    let class = jvm.lookup_class(class_name).ok_or("interface receiver class not found")?;
    if let Some(source) = class.source {
        for index in &source.interfaces {
            let name = lookup_class_name(&source.constant_pool, *index as usize)?;
            if implements(jvm, name, interface, seen)? { return Ok(true); }
        }
    }
    if let Some(parent) = &class.super_class {
        return implements(jvm, parent, interface, seen);
    }
    Ok(false)
}

pub(super) fn invoke(pool: &ConstantPool, index: usize, count: u8, frame: &mut Frame, jvm: &RuntimeConst) -> Result<RuntimeValue, String> {
    let (owner, name_type) = match constant_pool_lookup(pool, index) {
        Some(ConstantPoolEntry::InterfaceMethodref(owner, name_type)) => (*owner, *name_type),
        _ => return Err("invokeinterface requires an InterfaceMethodref".to_string()),
    };
    let interface = lookup_class_name(pool, owner as usize)?;
    let (name, descriptor) = match constant_pool_lookup(pool, name_type as usize) {
        Some(ConstantPoolEntry::NameAndType { name_index, descriptor_index }) => (
            lookup_utf8_constant(pool, *name_index as usize).ok_or("invalid interface method name")?,
            lookup_utf8_constant(pool, *descriptor_index as usize).ok_or("invalid interface descriptor")?,
        ),
        _ => return Err("invalid interface method name and type".to_string()),
    };
    let descriptor = parse_method_descriptor(descriptor)?;
    let slots = 1 + descriptor.parameters.iter().map(|parameter| match parameter {
        Descriptor::Long | Descriptor::Double => 2,
        _ => 1,
    }).sum::<usize>();
    if count as usize != slots {
        return Err("invalid invokeinterface argument count".to_string());
    }
    let mut arguments = Vec::new();
    for _ in &descriptor.parameters {
        arguments.push(frame.pop_value_force()?);
    }
    arguments.reverse();
    let receiver = frame.pop_value_force()?;
    let class_name = match &receiver {
        RuntimeValue::Object(object) => object.borrow().class.clone(),
        RuntimeValue::Null => {
            let class = jvm.lookup_class("java/lang/NullPointerException").ok_or("missing NullPointerException")?;
            *jvm.pending_exception.borrow_mut() = Some(RuntimeValue::Object(
                rc::Rc::new(cell::RefCell::new(class.create_object()))
            ));
            return Ok(RuntimeValue::Void);
        },
        _ => return Err("unsupported invokeinterface receiver".to_string()),
    };
    if !implements(jvm, &class_name, interface, &mut Vec::new())? {
        return Err("receiver does not implement interface".to_string());
    }
    let mut current = Some(class_name.as_str());
    while let Some(name_of_class) = current {
        let class = jvm.lookup_class(name_of_class).ok_or("interface receiver class not found")?;
        if let Some(method) = class.methods.get(name) {
            return match method {
                JVMMethod::Native(function) => {
                    arguments.insert(0, receiver);
                    Ok(function(&arguments))
                },
                JVMMethod::Bytecode(info, method_pool) => {
                    let mut callee = create_frame(info)?;
                    if callee.locals.len() < slots {
                        return Err("interface method has insufficient local slots".to_string());
                    }
                    callee.locals[0] = receiver;
                    let mut slot = 1;
                    for (argument, parameter) in arguments.into_iter().zip(&descriptor.parameters) {
                        callee.locals[slot] = argument;
                        slot += if matches!(parameter, Descriptor::Long | Descriptor::Double) { 2 } else { 1 };
                    }
                    do_execute_method(info, method_pool, &mut callee, jvm)
                },
            };
        }
        current = class.super_class.as_deref();
    }
    Err("interface implementation not found (default methods are unsupported)".to_string())
}
