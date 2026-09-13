use std::collections::HashMap;
use std::rc;
use std::cell;
use std::fmt;

use crate::debug;
use super::data::*;

// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5
pub mod opcodes {
    pub const ICONST0:u8 = 0x3; // iconst_0
    pub const ICONST1:u8 = 0x4; // iconst_1
    pub const ICONST2:u8 = 0x5; // iconst_2
    pub const ICONST3:u8 = 0x6; // iconst_3
    pub const ICONST4:u8 = 0x7; // iconst_4
    pub const ICONST5:u8 = 0x8; // iconst_5
    pub const DCONST0:u8 = 0x0e; // dconst_0
    pub const DCONST1:u8 = 0x0f; // dconst_1
    pub const DLOAD:u8 = 0x18; // dload
    pub const DLOAD0:u8 = 0x26; // dload_0
    pub const DLOAD3:u8 = 0x29; // dload_3
    pub const DALOAD:u8 = 0x31; // daload
    pub const DSTORE:u8 = 0x39; // dstore
    pub const DSTORE0:u8 = 0x47; // dstore_0
    pub const DSTORE3:u8 = 0x4a; // dstore_3
    pub const DASTORE:u8 = 0x52; // dastore
    pub const DADD:u8 = 0x63; // dadd
    pub const I2D:u8 = 0x87; // i2d
    pub const ACONSTNULL:u8 = 0x01; // aconst_null
    pub const BALOAD:u8 = 0x33; // baload
    pub const BASTORE:u8 = 0x54; // bastore
    pub const AALOAD:u8 = 0x32; // aaload
    pub const AASTORE:u8 = 0x53; // aastore
    pub const ANEWARRAY:u8 = 0xbd; // anewarray
    pub const NEWARRAY:u8 = 0xbc; // newarray
    pub const ARRAYLENGTH:u8 = 0xbe; // arraylength
    pub const PUSHBYTE:u8 = 0x10; // bipush
    pub const PUSHRUNTIMECONSTANT:u8 = 0x12; // ldc
    pub const ILOAD:u8 = 0x15; // iload
    pub const ILOAD0:u8 = 0x1a; // iload_0
    pub const ILOAD1:u8 = 0x1b; // iload_1
    pub const ILOAD2:u8 = 0x1c; // iload_2
    pub const ILOAD3:u8 = 0x1d; // iload_3
    pub const ALOAD0:u8 = 0x2a; // aload_0
    pub const ALOAD1:u8 = 0x2b; // aload_1
    pub const ISTORE:u8 = 0x36; // istore
    pub const ISTORE0:u8 = 0x3b; // istore_0
    pub const ISTORE1:u8 = 0x3c; // istore_1
    pub const ISTORE2:u8 = 0x3d; // istore_2
    pub const ISTORE3:u8 = 0x3e; // istore_3
    pub const ASTORE0:u8 = 0x4b; // astore_0
    pub const ASTORE1:u8 = 0x4c; // astore_1
    pub const DUP:u8 = 0x59; // dup
    pub const IADD:u8 = 0x60; // iadd
    pub const IMUL:u8 = 0x68; // imul
    pub const IDIV:u8 = 0x6c; // idiv
    pub const IINC:u8 = 0x84; // iinc
    pub const TABLESWITCH:u8 = 0xaa; // tableswitch
    pub const IFICOMPAREGREATEREQUAL:u8 = 0xa2; // if_icmpge
    pub const GOTO:u8 = 0xa7; // goto
    pub const IRETURN:u8 = 0xac; // ireturn
    pub const ARETURN:u8 = 0xb0; // areturn
    pub const RETURN:u8 = 0xb1; // return
    pub const GETSTATIC:u8 = 0xb2; // getstatic
    pub const GETFIELD:u8 = 0xb4; // getfield
    pub const PUTFIELD:u8 = 0xb5; // putfield
    pub const INVOKEVIRTUAL:u8 = 0xb6; // invokevirtual
    pub const INVOKESPECIAL:u8 = 0xb7; // invokespecial
    pub const INVOKESTATIC:u8 = 0xb8; // invokestatic
    pub const NEW:u8 = 0xbb; // new
    pub const ATHROW:u8 = 0xbf; // athrow
}

mod array_types {
    pub const BOOLEAN: u8 = 4;
    pub const DOUBLE: u8 = 7;
    pub const BYTE: u8 = 8;

    pub fn is_supported(atype: u8) -> bool {
        matches!(atype, BOOLEAN | DOUBLE | BYTE)
    }
}

#[derive(Clone)]
pub enum RuntimeValue{
    Int(i64),
    Long(i64),
    Float(f32),
    Double(f64),
    DoubleArray(rc::Rc<cell::RefCell<Vec<f64>>>),
    ByteArray(rc::Rc<cell::RefCell<JVMByteArray>>),
    ReferenceArray(rc::Rc<cell::RefCell<JVMReferenceArray>>),
    Null,
    Void,
    String(String),
    Object(rc::Rc<cell::RefCell<JVMObject>>),
}

pub struct JVMByteArray {
    is_boolean: bool,
    values: Vec<i8>,
}

pub struct JVMReferenceArray {
    component_class: String,
    values: Vec<RuntimeValue>,
}

/*
impl Clone for RuntimeValue {
    fn clone(self: &RuntimeValue) -> RuntimeValue {
        match self {
            RuntimeValue::Int(i) => RuntimeValue::Int(*i),
            RuntimeValue::Long(i) => RuntimeValue::Long(*i),
            RuntimeValue::Float(i) => RuntimeValue::Float(*i),
            RuntimeValue::Double(i) => RuntimeValue::Double(*i),
            RuntimeValue::Void => RuntimeValue::Void,
            RuntimeValue::String(s) => RuntimeValue::String(s.clone()),
            RuntimeValue::Object(object) => RuntimeValue::Object(object.clone()),
        }
    }
}
*/

impl fmt::Debug for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Int(value) => {
                write!(f, "Int({})", value)
            },
            RuntimeValue::Long(value) => {
                write!(f, "Long({})", value)
            },
            RuntimeValue::Float(value) => {
                write!(f, "Float({})", value)
            },
            RuntimeValue::Double(value) => {
                write!(f, "Double({})", value)
            },
            RuntimeValue::DoubleArray(values) => {
                write!(f, "DoubleArray({:?})", values.borrow())
            },
            RuntimeValue::ByteArray(array) => {
                write!(f, "ByteArray({:?})", array.borrow().values)
            },
            RuntimeValue::ReferenceArray(array) => {
                let array = array.borrow();
                write!(f, "ReferenceArray({}, length={})", array.component_class, array.values.len())
            },
            RuntimeValue::Null => write!(f, "Null"),
            RuntimeValue::Void => {
                write!(f, "Void")
            },
            RuntimeValue::String(value) => {
                write!(f, "String({})", value)
            },
            RuntimeValue::Object(value) => {
                write!(f, "Object({:?})", value.borrow().class)
            },
        }
    }
}

enum JVMMethod<'a>{
    Native(fn(&[RuntimeValue]) -> RuntimeValue),
    Bytecode(&'a MethodInfo, &'a ConstantPool),
}

fn create_jvm_class(jvmclass: &JVMClassFile) -> Result<JVMClass, String> {
    match constant_pool_lookup(&jvmclass.constant_pool, jvmclass.this_class as usize) {
        Some(ConstantPoolEntry::Classref(class_index)) => {
            match constant_pool_lookup(&jvmclass.constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Utf8(class_name)) => {
                    let mut methods = HashMap::new();

                    for method in jvmclass.methods.iter() {
                        let method_name = lookup_method_name(&jvmclass.constant_pool, method.name_index as usize)?;
                        methods.insert(method_name.to_string(), JVMMethod::Bytecode(method, &jvmclass.constant_pool));
                    }

                    return Ok(JVMClass{
                        class: class_name.to_string(),
                        super_class: if jvmclass.super_class == 0 { None } else {
                            Some(lookup_class_name(&jvmclass.constant_pool, jvmclass.super_class as usize)?.to_string())
                        },
                        methods: methods,
                        fields: HashMap::new(),
                    })
                },
                _ => {
                    return Err("Invalid name reference".to_string());
                }
            }
        },
        _ => {
            return Err("Invalid class reference".to_string());
        }
    }

}

struct JVMClass<'a>{
    class: String,
    super_class: Option<String>,
    methods: HashMap<String, JVMMethod<'a>>,
    fields: HashMap<String, RuntimeValue>,
}

pub struct JVMObject{
    class: String,
    fields: HashMap<String, RuntimeValue>,
}

impl Clone for JVMObject {
    fn clone(&self) -> Self {
        return JVMObject{
            class: self.class.clone(),
            fields: self.fields.clone(),
        }
    }
}

impl <'a>JVMClass<'a>{
    fn create_object(self: &JVMClass<'a>) -> JVMObject {
        return JVMObject{
            class: self.class.clone(),
            fields: HashMap::new(),
        }
    }
}

struct Frame {
    stack: Vec<RuntimeValue>,
    locals: Vec<RuntimeValue>,
}

struct RuntimeConst<'a> {
    classes: HashMap<String, JVMClass<'a>>,
    pending_exception: cell::RefCell<Option<RuntimeValue>>,
}

fn lookup_class_name(pool: &ConstantPool, index: usize) -> Result<&str, String> {
    if let Some(ConstantPoolEntry::Classref(name)) = constant_pool_lookup(pool, index) {
        if let Some(name) = lookup_utf8_constant(pool, *name as usize) {
            return Ok(name);
        }
    }
    Err(format!("invalid class reference {}", index))
}

fn reference_assignable(jvm: &RuntimeConst, value: &RuntimeValue, target: &str) -> bool {
    match value {
        RuntimeValue::Null => true,
        RuntimeValue::Object(object) => is_instance_of(jvm, &object.borrow().class, target),
        RuntimeValue::String(_) => target == "java/lang/String" || target == "java/lang/Object",
        RuntimeValue::DoubleArray(_) => target == "[D" || array_supertype(target),
        RuntimeValue::ByteArray(array) => {
            target == (if array.borrow().is_boolean { "[Z" } else { "[B" }) || array_supertype(target)
        },
        RuntimeValue::ReferenceArray(array) => {
            if array_supertype(target) { return true; }
            let component = array.borrow().component_class.clone();
            let descriptor = if component.starts_with('[') {
                format!("[{}", component)
            } else { format!("[L{};", component) };
            reference_class_assignable(jvm, &descriptor, target)
        },
        _ => false,
    }
}

fn array_supertype(target: &str) -> bool {
    matches!(target, "java/lang/Object" | "java/lang/Cloneable" | "java/io/Serializable")
}

fn reference_class_assignable(jvm: &RuntimeConst, source: &str, target: &str) -> bool {
    if source == target { return true; }
    if let Some(source) = source.strip_prefix('[') {
        if array_supertype(target) { return true; }
        if let Some(target) = target.strip_prefix('[') {
            let component = |s: &str| -> String {
                s.strip_prefix('L').and_then(|s| s.strip_suffix(';')).unwrap_or(s).to_string()
            };
            return reference_class_assignable(jvm, &component(source), &component(target));
        }
        return false;
    }
    is_instance_of(jvm, source, target)
}

fn is_instance_of(jvm: &RuntimeConst, class_name: &str, target: &str) -> bool {
    let mut current = Some(class_name);
    while let Some(name) = current {
        if name == target { return true; }
        current = jvm.lookup_class(name).and_then(|class| class.super_class.as_deref());
    }
    false
}

impl <'a, 'b: 'a>RuntimeConst<'a> {
    fn lookup_class(self: &RuntimeConst<'a>, class_name: &str) -> Option<&JVMClass> {
        return self.classes.get(class_name);
    }

    fn add_class(self: &mut RuntimeConst<'a>, jvm_class: JVMClass<'b>){
        self.classes.insert(jvm_class.class.clone(), jvm_class);
    }
}

impl Frame {
    fn as_ref(self: &mut Frame) -> &mut Frame {
        return self
    }

    fn push_value(self: &mut Frame, value: RuntimeValue) {
        self.stack.push(value);
    }

    fn pop_value(self: &mut Frame) -> Option<RuntimeValue> {
        return self.stack.pop();
    }

    fn pop_value_force(self: &mut Frame) -> Result<RuntimeValue, String> {
        match self.stack.pop() {
            Some(value) => {
                return Ok(value);
            },
            None => {
                return Err("Stack underflow".to_string());
            }
        }
    }
}

fn invoke_static(constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, method_index: usize) -> Result<RuntimeValue, String> {
    match constant_pool_lookup(constant_pool, method_index) {
        Some(ConstantPoolEntry::Methodref(class_index, name_and_type_index)) => {
            match constant_pool_lookup(constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Classref(class_index)) => {
                    match constant_pool_lookup(constant_pool, *class_index as usize) {
                        Some(ConstantPoolEntry::Utf8(class_name)) => {
                            debug!("Invoke method on class {}", class_name);

                            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                                    match constant_pool_lookup(constant_pool, *name_index as usize) {
                                        Some(ConstantPoolEntry::Utf8(method_name)) => {
                                            debug!("invoke static method {} on class {}", method_name, class_name);

                                            if let Some(descriptor) = lookup_utf8_constant(constant_pool, *descriptor_index as usize) {
                                                let method_descriptor = parse_method_descriptor(descriptor)?;
                                                let mut locals = Vec::new();
                                                for _i in 0..method_descriptor.parameters.len() {
                                                    locals.push(frame.pop_value_force()?);
                                                }
                                                locals.reverse();

                                                match jvm.lookup_class(class_name) {
                                                    Some(class) => {
                                                        match class.methods.get(method_name) {
                                                            Some(method) => {
                                                                match method {
                                                                    JVMMethod::Native(f) => {
                                                                        debug!("invoke native method");
                                                                        return Ok(f(locals.as_slice()));
                                                                    },
                                                                    JVMMethod::Bytecode(info, constant_pool) => {
                                                                        debug!("invoke bytecode method stack size {}", frame.stack.len());

                                                                        if let Some(AttributeKind::Code { max_stack: _, max_locals, code: _, exception_table: _, attributes: _ }) = lookup_code_attribute(info) {
                                                                            for _i in 0..((*max_locals as usize) - locals.len()) {
                                                                                locals.push(RuntimeValue::Int(0));
                                                                            }
                                                                        }

                                                                        let mut new_frame = create_frame(info)?;
                                                                        new_frame.locals = locals;
                                                                        return do_execute_method(&info, constant_pool, &mut new_frame, jvm);
                                                                    }
                                                                }
                                                            },
                                                            None => {
                                                                debug!("  Unknown method {}", method_name);
                                                            }
                                                        }
                                                    },
                                                    _ => {
                                                        debug!("  Unknown class {}", class_name);
                                                    }
                                                }
                                            } else {
                                                return Err(format!("could not find descriptor {}", *descriptor_index))
                                            }
                                        },
                                        _ => {
                                            return Err("Invalid name and type".to_string());
                                        }
                                    }
                                },
                                _ => {
                                    return Err("Invalid name and type".to_string());
                                }
                            }
                        },
                        _ => {
                            return Err("Invalid classref".to_string());
                        }
                    }
                },
                _ => {
                    return Err("Invalid classref".to_string());
                }
            }
        },
        _ => {
            return Err("Invalid methodref".to_string());
        }
    }
    return Err("error invoking method".to_string());
}

fn invoke_special(constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, method_index: usize) -> Result<(), String> {
    match constant_pool_lookup(constant_pool, method_index) {
        Some(ConstantPoolEntry::Methodref(class_index, name_and_type_index)) => {
            match constant_pool_lookup(constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Classref(class_index)) => {
                    match constant_pool_lookup(constant_pool, *class_index as usize) {
                        Some(ConstantPoolEntry::Utf8(class_name)) => {
                            debug!("Invoke method on class {}", class_name);

                            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                                    match constant_pool_lookup(constant_pool, *name_index as usize) {
                                        Some(ConstantPoolEntry::Utf8(name)) => {
                                            debug!("  method name={}", name);

                                            debug!("  frame stack size {}", frame.stack.len());
                                            if let Some(descriptor) = lookup_utf8_constant(constant_pool, *descriptor_index as usize) {
                                                debug!("  method descriptor={}", descriptor);
                                                let method_descriptor = parse_method_descriptor(descriptor)?;
                                                let mut locals = Vec::new();
                                                for i in 0..method_descriptor.parameters.len() {
                                                    locals.push(frame.pop_value_force()?);
                                                }
                                                let object_arg = frame.pop_value_force()?;

                                                match object_arg {
                                                    RuntimeValue::Object(object) => {
                                                        debug!("  popped object class '{}'", object.borrow().class);

                                                        match jvm.lookup_class(class_name) {
                                                            Some(class) => {
                                                                match class.methods.get(name) {
                                                                    Some(method) => {
                                                                        locals.push(RuntimeValue::Object(object));
                                                                        locals.reverse();

                                                                        match method {
                                                                            JVMMethod::Native(f) => {
                                                                                debug!("invoke native method");
                                                                                f(&locals.as_slice());
                                                                                return Ok(());
                                                                            },
                                                                            JVMMethod::Bytecode(info, constant_pool) => {
                                                                                debug!("invoke bytecode method '{}'", name);
                                                                                let mut new_frame = create_frame(info)?;
                                                                                locals.resize(new_frame.locals.len(), RuntimeValue::Void);
                                                                                new_frame.locals = locals;
                                                                                do_execute_method(&info, constant_pool, &mut new_frame, jvm)?;
                                                                                return Ok(());
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug!("  Unknown method {}", name);
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                debug!("could not find class with name {}", class_name);
                                                            }
                                                        }
                                                    },
                                                    value => {
                                                        return Err(format!("wrong value type on stack: {:?}", value));
                                                    }
                                                }
                                            } else {
                                                return Err(format!("could not find method descriptor {}", *descriptor_index));
                                            }
                                        },
                                        _ => {
                                            debug!("  Unknown name index {}", *name_index);
                                        }
                                    }
                                },
                                _ => {
                                    debug!("Unknown name and type index {}", *name_and_type_index);
                                }
                            }

                        },
                        _ => {
                            debug!("Unknown class index {}", class_index);
                        }
                    }
                },
                _ => {
                    debug!("Unknown class index {}", class_index);
                }
            }
        }
        _ => {
            debug!("Unknown method index {}", method_index);
        }
    }

    return Err(format!("unable to find method index {}", method_index).to_string())

}

fn invoke_virtual(constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, method_index: usize) -> Result<RuntimeValue, String> {
    // FIXME: handle polymorphic methods: https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-2.html#jvms-2.9.3

    match constant_pool_lookup(constant_pool, method_index) {
        Some(ConstantPoolEntry::Methodref(class_index, name_and_type_index)) => {
            match constant_pool_lookup(constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Classref(class_index)) => {
                    match constant_pool_lookup(constant_pool, *class_index as usize) {
                        Some(ConstantPoolEntry::Utf8(class_name)) => {
                            debug!("Invoke method on class {}", class_name);

                            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                                    match constant_pool_lookup(constant_pool, *name_index as usize) {
                                        Some(ConstantPoolEntry::Utf8(name)) => {
                                            debug!("  method name={}", name);

                                            debug!("  frame stack size {}", frame.stack.len());
                                            if let Some(descriptor) = lookup_utf8_constant(constant_pool, *descriptor_index as usize) {
                                                debug!("  method descriptor={}", descriptor);
                                                let method_descriptor = parse_method_descriptor(descriptor)?;
                                                let mut locals = Vec::new();
                                                for i in 0..method_descriptor.parameters.len() {
                                                    locals.push(frame.pop_value_force()?);
                                                }

                                                match frame.pop_value() {
                                                    Some(RuntimeValue::Object(object)) => {
                                                        debug!("  popped object class '{}'", object.borrow().class);

                                                        match jvm.lookup_class(class_name) {
                                                            Some(class) => {

                                                                /* push `this' pointer */
                                                                locals.push(RuntimeValue::Object(object));
                                                                locals.reverse();

                                                                match class.methods.get(name) {
                                                                    Some(method) => {
                                                                        match method {
                                                                            JVMMethod::Native(f) => {
                                                                                debug!("invoke native method");
                                                                                return Ok(f(&locals.as_slice()));
                                                                            },
                                                                            JVMMethod::Bytecode(info, constant_pool) => {
                                                                                debug!("invoke bytecode method '{}'", name);
                                                                                let mut new_frame = create_frame(info)?;

                                                                                // fill in the rest of the locals array with 0
                                                                                if let Some(AttributeKind::Code { max_stack: _, max_locals, code: _, exception_table: _, attributes: _ }) = lookup_code_attribute(info) {
                                                                                    for _i in 0..((*max_locals as usize) - locals.len()) {
                                                                                        locals.push(RuntimeValue::Int(0));
                                                                                    }
                                                                                }

                                                                                new_frame.locals = locals;
                                                                                return do_execute_method(&info, constant_pool, &mut new_frame, jvm)
                                                                            }
                                                                        }
                                                                    }
                                                                    None => {
                                                                        debug!("  Unknown method {}", name);
                                                                    }
                                                                }
                                                            },
                                                            _ => {
                                                                debug!("could not find class with name {}", class_name);
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        return Err("no value on stack".to_string());
                                                    }
                                                    Some(value) => {
                                                        return Err(format!("wrong value type on stack: {:?}", value));
                                                    }
                                                }
                                            } else {
                                                return Err(format!("could not find method descriptor {}", *descriptor_index));
                                            }
                                        },
                                        _ => {
                                            debug!("  Unknown name index {}", *name_index);
                                        }
                                    }
                                },
                                _ => {
                                    debug!("Unknown name and type index {}", *name_and_type_index);
                                }
                            }

                        },
                        _ => {
                            debug!("Unknown class index {}", class_index);
                        }
                    }
                },
                _ => {
                    debug!("Unknown class index {}", class_index);
                }
            }
        }
        _ => {
            debug!("Unknown method index {}", method_index);
        }
    }

    return Err(format!("unable to find method index {}", method_index).to_string())
}

fn op_getstatic(constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, field_index: usize) -> Result<(), String> {
    match constant_pool_lookup(constant_pool, field_index) {
        Some(ConstantPoolEntry::Fieldref{class_index, name_and_type_index}) => {
            debug!("  class={}", class_index);
            debug!("  name_and_type={}", name_and_type_index);

            match constant_pool_lookup(constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Classref(index)) => {
                    debug!("  classref={}", index);
                    match lookup_utf8_constant(constant_pool, *index as usize) {
                        Some(class_name) => {
                            debug!("  class_name={}", class_name);

                            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                                    debug!("  name_index={}", name_index);
                                    debug!("  descriptor_index={}", descriptor_index);

                                    match lookup_utf8_constant(constant_pool, *name_index as usize) {
                                        Some(name) => {
                                            debug!("  name={}", name);

                                            match jvm.lookup_class(class_name) {
                                                Some(class) => {
                                                    match class.fields.get(name) {
                                                        Some(value) => {
                                                            // debug!(" pushing value");
                                                            frame.push_value(value.clone());
                                                            return Ok(());
                                                        },
                                                        None => {
                                                            debug!("  Unknown field {}", name);
                                                        }
                                                    }
                                                },
                                                None => {
                                                    debug!("  Unknown class {}", class_name);
                                                }
                                            }
                                        },
                                        None => {
                                            debug!("  Unknown name index {}", *name_index);
                                        }
                                    }

                                    match lookup_utf8_constant(constant_pool, *descriptor_index as usize) {
                                        Some(descriptor) => {
                                            debug!("  descriptor={}", descriptor);
                                        },
                                        None => {
                                            debug!("  Unknown descriptor index {}", *descriptor_index);
                                        }
                                    }

                                },
                                _ => {
                                    debug!("  Unknown name and type index {}", *name_and_type_index);
                                }
                            }

                        },
                        _ => {
                            debug!("  Unknown classref {}", *index);
                        }
                    }
                },
                _ => {
                    debug!("  Unknown classref {}", class_index);
                }
            }

        },
        Some(entry) => {
            debug!("  {}", entry.name());
        },
        None => {
            debug!("  Unknown constant pool entry {}", field_index);
        }
    }

    return Err(format!("error in getstatic with index {}", field_index).to_string());
}

fn push_runtime_constant(constant_pool: &ConstantPool, frame: &mut Frame, index: usize) -> Result<(), String> {
    if index > 0 && index < constant_pool.len() {
        match constant_pool_lookup(constant_pool, index) {
            Some(ConstantPoolEntry::Utf8(name)) => {
                debug!("Pushing constant utf8 {}", name);
            },
            Some(ConstantPoolEntry::Stringref(string_index)) => {
                debug!("Pushing constant string {}", string_index);
                match constant_pool_lookup(constant_pool, *string_index as usize) {
                    Some(ConstantPoolEntry::Utf8(name)) => {
                        debug!("Pushing constant utf8 '{}'", name);
                        frame.push_value(RuntimeValue::String(name.clone()));
                        return Ok(());
                    },
                    None => {
                        debug!("no such index {}", string_index);
                    }
                    _ => {
                        debug!("constant pool index {} is invalid", string_index);
                    }
                }
            },
            _ => {
                debug!("ERROR: unhandled constant {}", &constant_pool[index-1].name());
            }
        }
    } else {
        return Err(format!("constant index {} out of range", index).to_string());
    }

    return Err("error with push constant".to_string());
}

fn do_iop(frame: &mut Frame, op: fn(i64, i64) -> i64) -> Result<RuntimeValue, String> {
    let value1 = frame.pop_value_force()?;
    let value2 = frame.pop_value_force()?;
    match value1 {
        RuntimeValue::Int(i1) => {
            match value2 {
                RuntimeValue::Int(i2) => {
                    debug!("  iop {} {} = {}", i1, i2, op(i1, i2));
                    return Ok(RuntimeValue::Int(op(i1, i2)));
                },
                _ => {
                    return Err("invalid value type for integer op".to_string());
                }
            }
        },
        _ => {
            return Err("invalid value type for integer op".to_string());
        }
    }
}

fn create_new_object(constant_pool: &ConstantPool, jvm: &RuntimeConst, index: usize) -> Result<RuntimeValue, String> {
    match constant_pool_lookup(constant_pool, index) {
        Some(ConstantPoolEntry::Classref(class_index)) => {
            match constant_pool_lookup(constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Utf8(class_name)) => {
                    debug!("  class_name={}", class_name);
                    match jvm.lookup_class(class_name) {
                        Some(class) => {
                            return Ok(RuntimeValue::Object(rc::Rc::new(cell::RefCell::new(class.create_object()))))
                        },
                        None => {
                            return Err(format!("no such class named '{}'", class_name).to_string());
                        }
                    }
                },
                _ => {
                    debug!("  Unknown classref {}", *class_index);
                }
            }
        },
        None => {
            return Err(format!("unknown classref {}", index))
        }
        _ => {
            return Err(format!("index {} was not a classref", index))
        }
    }

    return Err("error with create object".to_string());
}

fn lookup_code_attribute(method: &MethodInfo) -> Option<&AttributeKind> {
    for i in 0..method.attributes.len() {
        match &method.attributes[i] {
            AttributeKind::Code { max_stack: _, max_locals: _, code: _, exception_table: _, attributes: _ } => {
                return Some(&method.attributes[i]);
            },
            _ => {
            }
        }
    }

    return None;
}

fn putfield(constant_pool: &ConstantPool, jvm: &RuntimeConst, field_index: usize, object: RuntimeValue, field_value: RuntimeValue) -> Result<(), String> {
    match constant_pool_lookup(constant_pool, field_index) {
        Some(ConstantPoolEntry::Fieldref{class_index, name_and_type_index}) => {
            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                    match lookup_utf8_constant(constant_pool, *name_index as usize) {
                        Some(name) => {
                            match object {
                                RuntimeValue::Object(object) => {
                                    debug!("  set field {}.{} = {:?}", object.borrow().class, name, field_value);
                                    object.borrow_mut().fields.insert(name.to_string(), field_value);
                                    return Ok(());
                                },
                                _ => {
                                    return Err(format!("objectref was not an object: {:?}", object))
                                }
                            }
                        },
                        _ => {
                            return Err(format!("unknown name index {}", name_index))
                        }
                    }
                },
                _ => {
                    return Err(format!("unknown name and type index {}", name_and_type_index))
                }
            }
        },
        _ => {
            return Err(format!("unknown fieldref {}", field_index))
        }
    }
}

fn getfield(constant_pool: &ConstantPool, jvm: &RuntimeConst, field_index: usize, object: RuntimeValue) -> Result<RuntimeValue, String> {
    match constant_pool_lookup(constant_pool, field_index) {
        Some(ConstantPoolEntry::Fieldref{class_index, name_and_type_index}) => {
            match constant_pool_lookup(constant_pool, *name_and_type_index as usize) {
                Some(ConstantPoolEntry::NameAndType{name_index, descriptor_index}) => {
                    match lookup_utf8_constant(constant_pool, *name_index as usize) {
                        Some(name) => {
                            match object {
                                RuntimeValue::Object(object) => {
                                    if let Some(value) = object.borrow().fields.get(name) {
                                        return Ok(value.clone());
                                    } else {
                                        return Err(format!("no such field '{}' in class '{}'", name, object.borrow().class));
                                    }
                                },
                                _ => {
                                    return Err(format!("objectref was not an object: {:?}", object))
                                }
                            }
                        },
                        _ => {
                            return Err(format!("unknown name index {}", name_index))
                        }
                    }
                },
                _ => {
                    return Err(format!("unknown name and type index {}", name_and_type_index))
                }
            }
        },
        _ => {
            return Err(format!("unknown fieldref {}", field_index))
        }
    }
}

fn do_icompare(frame: &mut Frame, pc: usize, offset: i16, compare: fn(i64, i64) -> bool) -> Result<usize, String>{
    let value2 = frame.pop_value_force()?;
    let value1 = frame.pop_value_force()?;
    match (value1, value2) {
        (RuntimeValue::Int(i1), RuntimeValue::Int(i2)) => {
            if compare(i1, i2) {
                return Ok((pc as i16 + offset) as usize);
            }
        }
        _ => {
            return Err("invalid compare of non-int".to_string());
        }
    }

    return Ok(pc + 3)
}

fn make_int16(byte1:u8, byte2:u8) -> u16 {
    return ((byte1 as u16) << 8) | (byte2 as u16)
}

fn make_int32(byte1:u8, byte2:u8, byte3:u8, byte4:u8) -> u32 {
    let b1 = byte1 as u32;
    let b2 = byte2 as u32;
    let b3 = byte3 as u32;
    let b4 = byte4 as u32;
    return (b1 << 24) | (b2 << 16) | (b3 << 8) | b4;
}

fn do_execute_method(method: &MethodInfo, constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst) -> Result<RuntimeValue, String> {
    if let Some(AttributeKind::Code { max_stack, max_locals, code, exception_table, attributes }) = lookup_code_attribute(method) {
        // FIXME: create frame based on max_stack and max_locals
        debug!("Code attribute");
        debug!("  max_stack={}", max_stack);
        debug!("  max_locals={}", max_locals);
        debug!("  code={}", code.len());
        debug!("  exception_table={}", exception_table.len());
        debug!("  attributes={}", attributes.len());

        let mut pc = 0;
        while pc < code.len() {
            // println!("Opcopde {}: 0x{:x}", pc, code[pc]);
            let instruction_pc = pc;
            match code[pc] {
                opcodes::ACONSTNULL => {
                    frame.push_value(RuntimeValue::Null);
                    pc += 1;
                },
                opcodes::ANEWARRAY => {
                    let index = make_int16(code[pc + 1], code[pc + 2]) as usize;
                    let component_class = lookup_class_name(constant_pool, index)?.to_string();
                    let count = match frame.pop_value_force()? {
                        RuntimeValue::Int(count) if count >= 0 => count as usize,
                        _ => return Err("invalid array length".to_string()),
                    };
                    let mut values = Vec::new();
                    values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                    values.resize(count, RuntimeValue::Null);
                    frame.push_value(RuntimeValue::ReferenceArray(rc::Rc::new(cell::RefCell::new(
                        JVMReferenceArray { component_class, values }
                    ))));
                    pc += 3;
                },
                opcodes::AALOAD | opcodes::AASTORE => {
                    let stored = if code[pc] == opcodes::AASTORE {
                        Some(frame.pop_value_force()?)
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::ReferenceArray(array) => {
                            if let Some(value) = &stored {
                                let component_class = array.borrow().component_class.clone();
                                if !reference_assignable(jvm, value, &component_class) {
                                    return Err("incompatible reference array element".to_string());
                                }
                            }
                            let mut array = array.borrow_mut();
                            let slot = array.values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(slot.clone());
                            }
                        },
                        _ => return Err("reference array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::NEWARRAY => {
                    let atype = code[pc + 1];
                    if !array_types::is_supported(atype) {
                        return Err(format!("unsupported newarray type {}", atype));
                    }
                    let count = match frame.pop_value_force()? {
                        RuntimeValue::Int(count) if count >= 0 => count as usize,
                        _ => return Err("invalid array length".to_string()),
                    };
                    if atype == array_types::DOUBLE {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0.0);
                        frame.push_value(RuntimeValue::DoubleArray(rc::Rc::new(cell::RefCell::new(values))));
                    } else {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0);
                        frame.push_value(RuntimeValue::ByteArray(rc::Rc::new(cell::RefCell::new(
                            JVMByteArray { is_boolean: atype == array_types::BOOLEAN, values }
                        ))));
                    }
                    pc += 2;
                },
                opcodes::ARRAYLENGTH => {
                    let length = match frame.pop_value_force()? {
                        RuntimeValue::DoubleArray(values) => values.borrow().len(),
                        RuntimeValue::ByteArray(array) => array.borrow().values.len(),
                        RuntimeValue::ReferenceArray(array) => array.borrow().values.len(),
                        _ => return Err("arraylength requires an array".to_string()),
                    };
                    frame.push_value(RuntimeValue::Int(length as i64));
                    pc += 1;
                },
                opcodes::BALOAD | opcodes::BASTORE => {
                    let stored = if code[pc] == opcodes::BASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Int(value) => Some(value),
                            _ => return Err("bastore requires an integer".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::ByteArray(array) => {
                            let mut array = array.borrow_mut();
                            let is_boolean = array.is_boolean;
                            let slot = array.values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = if is_boolean { (value & 1) as i8 } else { value as i8 };
                            } else {
                                // Sign-extend bytes; boolean elements are always 0 or 1.
                                frame.push_value(RuntimeValue::Int(*slot as i64));
                            }
                        },
                        _ => return Err("byte or boolean array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::DALOAD | opcodes::DASTORE => {
                    let stored = if code[pc] == opcodes::DASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Double(value) => Some(value),
                            _ => return Err("dastore requires a double".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::DoubleArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(RuntimeValue::Double(*slot));
                            }
                        },
                        _ => return Err("double array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::DCONST0 | opcodes::DCONST1 => {
                    frame.push_value(RuntimeValue::Double((code[pc] - opcodes::DCONST0) as f64));
                    pc += 1;
                },
                opcodes::DLOAD | opcodes::DLOAD0..=opcodes::DLOAD3 => {
                    let (index, size) = if code[pc] == opcodes::DLOAD {
                        (code[pc + 1] as usize, 2)
                    } else { ((code[pc] - opcodes::DLOAD0) as usize, 1) };
                    match frame.locals.get(index) {
                        Some(RuntimeValue::Double(value)) => frame.push_value(RuntimeValue::Double(*value)),
                        _ => return Err("dload requires a double local".to_string()),
                    }
                    pc += size;
                },
                opcodes::DSTORE | opcodes::DSTORE0..=opcodes::DSTORE3 => {
                    let (index, size) = if code[pc] == opcodes::DSTORE {
                        (code[pc + 1] as usize, 2)
                    } else { ((code[pc] - opcodes::DSTORE0) as usize, 1) };
                    let value = frame.pop_value_force()?;
                    if !matches!(value, RuntimeValue::Double(_)) || index + 1 >= frame.locals.len() {
                        return Err("invalid dstore".to_string());
                    }
                    frame.locals[index] = value;
                    // Doubles occupy two local slots, but one operand-stack entry.
                    frame.locals[index + 1] = RuntimeValue::Void;
                    pc += size;
                },
                opcodes::I2D => {
                    match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => frame.push_value(RuntimeValue::Double(value as f64)),
                        _ => return Err("i2d requires an integer".to_string()),
                    }
                    pc += 1;
                },
                opcodes::DADD => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                            frame.push_value(RuntimeValue::Double(left + right));
                        },
                        _ => return Err("dadd requires doubles".to_string()),
                    }
                    pc += 1;
                },
                opcodes::ATHROW => {
                    let exception = frame.pop_value_force()?;
                    if !matches!(exception, RuntimeValue::Object(_)) {
                        return Err("athrow requires an object".to_string());
                    }
                    *jvm.pending_exception.borrow_mut() = Some(exception);
                },
                opcodes::ICONST0 => {
                    frame.push_value(RuntimeValue::Int(0));
                    pc += 1;
                },
                opcodes::ICONST1 => {
                    frame.push_value(RuntimeValue::Int(1));
                    pc += 1;
                },
                opcodes::ICONST2 => {
                    frame.push_value(RuntimeValue::Int(2));
                    pc += 1;
                },
                opcodes::ICONST3 => {
                    frame.push_value(RuntimeValue::Int(3));
                    pc += 1;
                },
                opcodes::ICONST4 => {
                    frame.push_value(RuntimeValue::Int(4));
                    pc += 1;
                },
                opcodes::ICONST5 => {
                    frame.push_value(RuntimeValue::Int(5));
                    pc += 1;
                },
                opcodes::PUSHBYTE => {
                    let value = code[pc + 1] as i8 as i64;
                    frame.push_value(RuntimeValue::Int(value));
                    pc += 2;
                },
                opcodes::ARETURN => {
                    let value = frame.pop_value_force()?;
                    match value {
                        RuntimeValue::Object(_) | RuntimeValue::String(_) | RuntimeValue::Null
                        | RuntimeValue::ReferenceArray(_) | RuntimeValue::DoubleArray(_)
                        | RuntimeValue::ByteArray(_) => return Ok(value),
                        _ => return Err("areturn requires a reference".to_string()),
                    }
                },
                opcodes::IRETURN => {
                    pc += 1;

                    // let value = frame.pop_value_force()?; 
                    // println!("returning value {:?}", value);

                    return Ok(frame.pop_value_force()?);
                    // return Ok(value);
                },
                opcodes::DUP => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.push_value(value.clone());
                    frame.push_value(value);
                },
                opcodes::ASTORE0 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[0] = value;
                },
                opcodes::ASTORE1 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[1] = value;
                },
                opcodes::ISTORE => {
                    let index = code[pc + 1] as usize;
                    let value = frame.pop_value_force()?;
                    frame.locals[index] = value;
                    pc += 2;
                },
                opcodes::ISTORE0 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[0] = value;
                },
                opcodes::ISTORE1 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[1] = value;
                },
                opcodes::ISTORE2 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[2] = value;
                },
                opcodes::ISTORE3 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[3] = value;
                },
                opcodes::ALOAD0 => {
                    pc += 1;
                    let value = frame.locals[0].clone();
                    frame.push_value(value);
                },
                opcodes::ALOAD1 => {
                    pc += 1;
                    let value = frame.locals[1].clone();
                    frame.push_value(value);
                },
                opcodes::TABLESWITCH => {
                    let original_pc = pc;

                    pc += 1;
                    let padding = pc % 4;
                    if padding != 0 {
                        pc += 4 - padding;
                    }

                    let default = make_int32(code[pc], code[pc+1], code[pc+2], code[pc+3]);
                    pc += 4;
                    let low = make_int32(code[pc], code[pc+1], code[pc+2], code[pc+3]) as i64;
                    pc += 4;
                    let high = make_int32(code[pc], code[pc+1], code[pc+2], code[pc+3]) as i64;
                    pc += 4;

                    // TODO: we don't really need to build this vector, we could just use the index
                    // to directly map into the correct code[] offset
                    let mut offsets = Vec::new();
                    for _ in low..=high {
                        let offset = make_int32(code[pc], code[pc+1], code[pc+2], code[pc+3]);
                        pc += 4;
                        offsets.push(offset);
                    }

                    let index = frame.pop_value_force()?;

                    match index {
                        RuntimeValue::Int(i) => {
                            if i < low || i > high {
                                pc = (original_pc as i32 + default as i32) as usize;
                            } else {
                                let offset = offsets[(i - low) as usize];
                                pc = (original_pc as i32 + offset as i32) as usize;
                            }
                        },
                        _ => {
                            return Err(format!("Invalid index for tableswitch: {:?}", index));
                        }
                    }
                },
                opcodes::IFICOMPAREGREATEREQUAL => {
                    pc = do_icompare(frame, pc, make_int16(code[pc+1], code[pc+2]) as i16, |i1, i2| i1 >= i2)?;
                },
                opcodes::GOTO => {
                    let old = pc;
                    let offset = make_int16(code[pc+1], code[pc+2]);
                    pc = (pc as i16 + offset as i16) as usize;
                },
                opcodes::ILOAD => {
                    let index = code[pc + 1] as usize;
                    let value = frame.locals[index].clone();
                    frame.push_value(value);
                    pc += 2;
                },
                opcodes::ILOAD0 => {
                    pc += 1;
                    let value = frame.locals[0].clone();
                    debug!("  load0: loading value {:?}", value);
                    frame.push_value(value);
                },
                opcodes::ILOAD1 => {
                    pc += 1;
                    let value = frame.locals[1].clone();
                    frame.push_value(value);
                },
                opcodes::ILOAD2 => {
                    pc += 1;
                    let value = frame.locals[2].clone();
                    frame.push_value(value);
                },
                opcodes::ILOAD3 => {
                    pc += 1;
                    let value = frame.locals[3].clone();
                    frame.push_value(value);
                },
                opcodes::IADD => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 + i2)?;
                    frame.stack.push(value);
                },
                opcodes::IMUL => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 * i2)?;
                    frame.stack.push(value);
                },
                opcodes::IDIV => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 / i2)?;
                    frame.stack.push(value);
                },
                opcodes::IINC => {
                    let index = code[pc + 1] as usize;
                    let value = frame.locals[index].clone();
                    let inc = code[pc + 2] as i64;
                    match value {
                        RuntimeValue::Int(i) => {
                            frame.locals[index] = RuntimeValue::Int(i + inc)
                        },
                        _ => {
                            return Err("inc on non-int".to_string());
                        }
                    }
                    pc += 3;
                },
                opcodes::NEW => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    frame.push_value(create_new_object(constant_pool, jvm, total)?);

                    pc += 3;
                },
                opcodes::GETFIELD => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;
                    let objectref = frame.pop_value_force()?;
                    frame.push_value(getfield(constant_pool, jvm, total, objectref)?);
                    pc += 3;
                },
                opcodes::PUTFIELD => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    let value = frame.pop_value_force()?;
                    let objectref = frame.pop_value_force()?;

                    putfield(constant_pool, jvm, total, objectref, value)?;

                    pc += 3;
                },
                opcodes::INVOKESPECIAL => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;
                    invoke_special(constant_pool, frame, jvm, total)?;
                    pc += 3;
                },
                opcodes::INVOKESTATIC => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    match invoke_static(constant_pool, frame, jvm, total)? {
                        RuntimeValue::Void => {},
                        r => {
                            debug!("got back value {:?}", r);
                            frame.stack.push(r);
                        }
                    }

                    pc += 3;
                },
                opcodes::GETSTATIC => {
                    debug!("Get static");
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    pc += 2;

                    op_getstatic(constant_pool, frame, jvm, total)?;

                    pc += 1;
                },
                opcodes::INVOKEVIRTUAL => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    match invoke_virtual(constant_pool, frame, jvm, total)? {
                        RuntimeValue::Void => {},
                        r => {
                            frame.stack.push(r);
                        }
                    }

                    pc += 3;
                },
                opcodes::RETURN => {
                    return Ok(RuntimeValue::Void);
                },
                opcodes::PUSHRUNTIMECONSTANT => {
                    let index = code[pc+1] as usize;
                    push_runtime_constant(constant_pool, frame, index)?;
                    pc += 2;
                },
                _ => {
                    return Err(format!("Unknown opcode pc={} opcode=0x{:x}", pc, code[pc]));
                }
            }

            let exception = jvm.pending_exception.borrow_mut().take();
            if let Some(exception) = exception {
                let class_name = match &exception {
                    RuntimeValue::Object(object) => object.borrow().class.clone(),
                    _ => unreachable!(),
                };
                let mut handler = None;
                for entry in exception_table {
                    if instruction_pc >= entry.start_pc as usize && instruction_pc < entry.end_pc as usize
                        && (entry.catch_type == 0 || is_instance_of(jvm, &class_name,
                            lookup_class_name(constant_pool, entry.catch_type as usize)?)) {
                        handler = Some(entry.handler_pc as usize);
                        break;
                    }
                }
                if let Some(target) = handler {
                    frame.stack.clear();
                    frame.push_value(exception);
                    pc = target;
                } else {
                    *jvm.pending_exception.borrow_mut() = Some(exception);
                    return Ok(RuntimeValue::Void);
                }
            }
        }

    } else {
        return Err("no code attribute".to_string());
    }

    return Ok(RuntimeValue::Void);
}

fn create_stdout_object() -> rc::Rc<cell::RefCell<JVMObject>> {
    return rc::Rc::new(cell::RefCell::new(JVMObject{
        class: "java/io/PrintStream".to_string(),
        fields: HashMap::new()
    }));
}

fn create_java_io_print_stream<'a>() -> JVMClass<'a> {
    let mut methods = HashMap::new();
    methods.insert("println".to_string(), JVMMethod::Native(|args: &[RuntimeValue]| {
        for arg in &args[1..] {
            match arg {
                RuntimeValue::String(s) => {
                    println!("{}", s);
                },
                RuntimeValue::Int(i) => {
                    println!("{}", i);
                },
                RuntimeValue::Double(value) => {
                    if value.is_infinite() {
                        println!("{}Infinity", if value.is_sign_negative() { "-" } else { "" });
                    } else {
                        println!("{:?}", value);
                    }
                },
                _ => {
                    println!("Unknown value type for println: {:?}", arg);
                }
            }
        }

        return RuntimeValue::Void;
    }));

    let fields = HashMap::new();

    return JVMClass{
        class: "java/io/PrintStream".to_string(),
        super_class: Some("java/lang/Object".to_string()),
        methods: methods,
        fields: fields,
    }
}

fn create_java_lang_system<'a>() -> JVMClass<'a> {
    let mut fields = HashMap::new();

    fields.insert("out".to_string(), RuntimeValue::Object(create_stdout_object()));

    let mut methods = HashMap::new();

    return JVMClass{
        class: "java/lang/System".to_string(),
        super_class: Some("java/lang/Object".to_string()),
        methods: methods,
        fields: fields
    };
}

fn create_frame(method: &MethodInfo) -> Result<Frame, String> {
    if let Some(AttributeKind::Code { max_stack: _, max_locals, code: _, exception_table: _, attributes: _ }) = lookup_code_attribute(method) {
        let mut locals = Vec::new();

        for _i in 0..*max_locals {
            locals.push(RuntimeValue::Void);
        }

        return Ok(Frame{
            stack: Vec::new(),
            locals,
        })
    }

    return Err("no code attribute".to_string());
}

fn create_java_lang_object<'a>() -> JVMClass<'a> {
    let mut fields = HashMap::new();
    let mut methods = HashMap::new();

    methods.insert("<init>".to_string(), JVMMethod::Native(|_args: &[RuntimeValue]| {
        return RuntimeValue::Void;
    }));

    return JVMClass{
        class: "java/lang/Object".to_string(),
        super_class: None,
        methods: methods,
        fields: fields,
    };
}

fn create_runtime_const<'a>() -> RuntimeConst<'a> {
    let mut classes = HashMap::new();

    classes.insert("java/lang/System".to_string(), create_java_lang_system());
    classes.insert("java/io/PrintStream".to_string(), create_java_io_print_stream());
    classes.insert("java/lang/Object".to_string(), create_java_lang_object());

    for (name, parent) in [("java/lang/Throwable", "java/lang/Object"),
                           ("java/lang/Exception", "java/lang/Throwable"),
                           ("java/lang/RuntimeException", "java/lang/Exception")] {
        let mut class = create_java_lang_object();
        class.class = name.to_string();
        class.super_class = Some(parent.to_string());
        classes.insert(name.to_string(), class);
    }

    return RuntimeConst{
        classes: classes,
        pending_exception: cell::RefCell::new(None),
    }
}

pub fn execute_class_file(path: &str, name: &str) -> Result<RuntimeValue, String> {
    let entry = parse_class_file(path).map_err(|err| err.to_string())?;
    let entry_name = lookup_class_name(&entry.constant_pool, entry.this_class as usize)?;
    let mut root = std::path::Path::new(path).to_path_buf();
    for _ in entry_name.split('/') { root.pop(); }
    let mut seen = std::collections::HashSet::new();
    seen.insert(entry_name.to_string());
    let mut classes = vec![entry];
    let mut index = 0;
    while index < classes.len() {
        let mut dependencies = Vec::new();
        for constant in &classes[index].constant_pool {
            if let ConstantPoolEntry::Classref(name_index) = constant {
                if let Some(name) = lookup_utf8_constant(&classes[index].constant_pool, *name_index as usize) {
                    if seen.insert(name.to_string()) {
                        let file = root.join(format!("{}.class", name));
                        if file.is_file() { dependencies.push(file); }
                    }
                }
            }
        }
        for file in dependencies {
            classes.push(parse_class_file(file.to_str().ok_or("invalid class path")?)
                .map_err(|err| err.to_string())?);
        }
        index += 1;
    }
    execute_method_with_classes(&classes[0], name, &classes[1..])
}

pub fn execute_method(jvm: &JVMClassFile, name: &str) -> Result<RuntimeValue, String> {
    execute_method_with_classes(jvm, name, &[])
}

pub fn execute_method_with_classes(jvm: &JVMClassFile, name: &str, classes: &[JVMClassFile]) -> Result<RuntimeValue, String> {
    // find method named 'name'
    // start executing byte code at that method

    for i in 0..jvm.methods.len() {
        /*
        match lookup_utf8_constant(jvm, jvm.methods[i].descriptor_index as usize) {
            Some(descriptor_name) => {
                println!("Method {} descriptor {}", i, descriptor_name);
            },
            None => {
                println!("Error: method {} descriptor index {} is invalid", i, jvm.methods[i].descriptor_index);
            }
        }
        */
        match lookup_utf8_constant(&jvm.constant_pool, jvm.methods[i].name_index as usize) {
            Some(method_name) => {
                debug!("Check method index={} name='{}' vs '{}'", i, method_name, name);
                if method_name == name {

                    let mut frame = create_frame(&jvm.methods[i])?;

                    let mut runtime = create_runtime_const();
                    runtime.add_class(create_jvm_class(jvm)?);
                    for class in classes {
                        runtime.add_class(create_jvm_class(class)?);
                    }

                    let result = do_execute_method(&jvm.methods[i], &jvm.constant_pool, &mut frame, &runtime)?;
                    if let Some(exception) = runtime.pending_exception.borrow_mut().take() {
                        return Err(format!("uncaught exception: {:?}", exception));
                    }
                    return Ok(result);
                }
            },
            None => {
                return Err("Error: method name index is invalid".to_string());
            }
        }
    }

    return Err("no such method found".to_string());
}

