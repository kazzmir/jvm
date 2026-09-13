use super::*;

fn string_constant(pool: &ConstantPool, index: u16) -> Result<&str, String> {
    match constant_pool_lookup(pool, index as usize) {
        Some(ConstantPoolEntry::Stringref(index)) => {
            lookup_utf8_constant(pool, *index as usize).ok_or("invalid concat string".to_string())
        },
        _ => Err("unsupported concat bootstrap constant".to_string()),
    }
}

// Native support for StringConcatFactory, not a general bootstrap-method linker.
// Initially supports int arguments and String bootstrap constants only.
pub(super) fn invoke_concat(pool: &ConstantPool, index: usize, frame: &mut Frame, jvm: &RuntimeConst) -> Result<(), String> {
    let (bootstrap, name_type) = match constant_pool_lookup(pool, index) {
        Some(ConstantPoolEntry::InvokeDynamic(bootstrap, name_type)) => (*bootstrap, *name_type),
        _ => return Err("invokedynamic requires an InvokeDynamic constant".to_string()),
    };
    let descriptor = match constant_pool_lookup(pool, name_type as usize) {
        Some(ConstantPoolEntry::NameAndType { descriptor_index, .. }) => {
            lookup_utf8_constant(pool, *descriptor_index as usize).ok_or("invalid dynamic descriptor")?
        },
        _ => return Err("invalid dynamic name and type".to_string()),
    };
    let source = jvm.classes.values().filter_map(|class| class.source)
        .find(|class| std::ptr::eq(&class.constant_pool, pool))
        .ok_or("missing invokedynamic defining class")?;
    let methods = source.attributes.iter().find_map(|attribute| match attribute {
        AttributeKind::BootstrapMethods(methods) => Some(methods),
        _ => None,
    }).ok_or("missing BootstrapMethods attribute")?;
    let (handle, arguments) = methods.get(bootstrap as usize).ok_or("invalid bootstrap index")?;
    let reference = match constant_pool_lookup(pool, *handle as usize) {
        // REF_invokeStatic
        Some(ConstantPoolEntry::MethodHandle(6, reference)) => *reference,
        _ => return Err("unsupported bootstrap method handle".to_string()),
    };
    let (owner, name_type) = match constant_pool_lookup(pool, reference as usize) {
        Some(ConstantPoolEntry::Methodref(owner, name_type)) => (*owner, *name_type),
        _ => return Err("invalid bootstrap method reference".to_string()),
    };
    let name = match constant_pool_lookup(pool, name_type as usize) {
        Some(ConstantPoolEntry::NameAndType { name_index, .. }) => {
            lookup_utf8_constant(pool, *name_index as usize).ok_or("invalid bootstrap name")?
        },
        _ => return Err("invalid bootstrap name and type".to_string()),
    };
    if lookup_class_name(pool, owner as usize)? != "java/lang/invoke/StringConcatFactory"
        || name != "makeConcatWithConstants" {
        return Err("unsupported invokedynamic bootstrap (only StringConcatFactory.makeConcatWithConstants is supported)".to_string());
    }
    let parameters = descriptor.strip_prefix('(')
        .and_then(|value| value.strip_suffix(")Ljava/lang/String;"))
        .ok_or("unsupported concat descriptor")?;
    if !parameters.bytes().all(|kind| kind == b'I') {
        return Err("unsupported concat parameter type (only int is supported)".to_string());
    }
    let recipe = string_constant(pool, *arguments.first().ok_or("missing concat recipe")?)?;
    let constants: Vec<&str> = arguments[1..].iter()
        .map(|index| string_constant(pool, *index)).collect::<Result<_, _>>()?;
    if recipe.chars().filter(|c| *c == '\u{1}').count() != parameters.len()
        || recipe.chars().filter(|c| *c == '\u{2}').count() != constants.len() {
        return Err("concat recipe argument count mismatch".to_string());
    }
    let start = frame.stack.len().checked_sub(parameters.len()).ok_or("Stack underflow")?;
    let values: Vec<String> = frame.stack[start..].iter().map(|value| match value {
        RuntimeValue::Int(value) => Ok((*value as i32).to_string()),
        _ => Err("concat requires an int argument".to_string()),
    }).collect::<Result<_, _>>()?;
    let mut values = values.iter();
    let mut constants = constants.iter();
    let mut output = String::new();
    for character in recipe.chars() {
        match character {
            '\u{1}' => output.push_str(values.next().ok_or("missing concat argument")?),
            '\u{2}' => output.push_str(constants.next().ok_or("missing concat constant")?),
            character => output.push(character),
        }
    }
    frame.stack.truncate(start);
    frame.push_value(RuntimeValue::String(rc::Rc::new(output)));
    Ok(())
}
