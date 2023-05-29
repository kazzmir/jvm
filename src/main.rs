use std::env;
use std::collections::HashMap;
use std::fmt;
use std::rc;
use std::cell;
use debug_print::debug_eprintln as debug;

mod jvm;

use jvm::data::*;

/*
fn lookup_method_name(jvm: &JVMClassFile, method_index: usize) -> Option<&str>{
    if method_index < jvm.methods.len() {
        let method = &jvm.methods[method_index];
        let name_index = method.name_index as usize;
        return lookup_utf8_constant(&jvm.constant_pool, name_index)
    }

    return None
}
*/

// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5
mod Opcodes {
    pub const IConst0:u8 = 0x3; // iconst_0
    pub const IConst1:u8 = 0x4; // iconst_1
    pub const IConst2:u8 = 0x5; // iconst_2
    pub const IConst3:u8 = 0x6; // iconst_3
    pub const IConst4:u8 = 0x7; // iconst_4
    pub const IConst5:u8 = 0x8; // iconst_5
    pub const PushByte:u8 = 0x10; // bipush
    pub const PushRuntimeConstant:u8 = 0x12; // ldc
    pub const ILoad:u8 = 0x15; // iload
    pub const ILoad0:u8 = 0x1a; // iload_0
    pub const ILoad1:u8 = 0x1b; // iload_1
    pub const ILoad2:u8 = 0x1c; // iload_2
    pub const ILoad3:u8 = 0x1d; // iload_3
    pub const ALoad0:u8 = 0x2a; // aload_0
    pub const ALoad1:u8 = 0x2b; // aload_1
    pub const IStore:u8 = 0x36; // istore
    pub const IStore0:u8 = 0x3b; // istore_0
    pub const IStore1:u8 = 0x3c; // istore_1
    pub const IStore2:u8 = 0x3d; // istore_2
    pub const IStore3:u8 = 0x3e; // istore_3
    pub const AStore1:u8 = 0x4c; // astore_1
    pub const Dup:u8 = 0x59; // dup
    pub const IAdd:u8 = 0x60; // iadd
    pub const IMul:u8 = 0x68; // imul
    pub const IDiv:u8 = 0x6c; // idiv
    pub const IInc:u8 = 0x84; // iinc
    pub const TableSwitch:u8 = 0xaa; // tableswitch
    pub const IfICompareGreaterEqual:u8 = 0xa2; // if_icmpge
    pub const Goto:u8 = 0xa7; // goto
    pub const IReturn:u8 = 0xac; // ireturn
    pub const Return:u8 = 0xb1; // return
    pub const GetStatic:u8 = 0xb2; // getstatic
    pub const GetField:u8 = 0xb4; // getfield
    pub const PutField:u8 = 0xb5; // putfield
    pub const InvokeVirtual:u8 = 0xb6; // invokevirtual
    pub const InvokeSpecial:u8 = 0xb7; // invokespecial
    pub const InvokeStatic:u8 = 0xb8; // invokestatic
    pub const New:u8 = 0xbb; // new
}

#[derive(Clone)]
enum RuntimeValue{
    Int(i64),
    Long(i64),
    Float(f32),
    Double(f64),
    Void,
    String(String),
    Object(rc::Rc<cell::RefCell<JVMObject>>),
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
    Bytecode(&'a MethodInfo),
}

fn lookup_method_name(constant_pool: &ConstantPool, index: usize) -> Result<String, String> {
    if let Some(method_name) = lookup_utf8_constant(constant_pool, index) {
        return Ok(method_name.to_string());
    }

    return Err(format!("no such method name at index {}", index));
}

fn createJvmClass(jvmclass: &JVMClassFile) -> Result<JVMClass, String> {
    match constant_pool_lookup(&jvmclass.constant_pool, jvmclass.this_class as usize) {
        Some(ConstantPoolEntry::Classref(class_index)) => {
            match constant_pool_lookup(&jvmclass.constant_pool, *class_index as usize) {
                Some(ConstantPoolEntry::Utf8(class_name)) => {
                    let mut methods = HashMap::new();

                    for method in jvmclass.methods.iter() {
                        let method_name = lookup_method_name(&jvmclass.constant_pool, method.name_index as usize)?;
                        methods.insert(method_name.to_string(), JVMMethod::Bytecode(method));
                    }

                    return Ok(JVMClass{
                        class: class_name.to_string(),
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
    methods: HashMap<String, JVMMethod<'a>>,
    fields: HashMap<String, RuntimeValue>,
}

struct JVMObject{
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
                                                for i in 0..method_descriptor.parameters.len() {
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
                                                                    JVMMethod::Bytecode(info) => {
                                                                        debug!("invoke bytecode method stack size {}", frame.stack.len());

                                                                        if let Some(AttributeKind::Code { max_stack, max_locals, code, exception_table, attributes }) = lookup_code_attribute(info) {
                                                                            for _i in 0..((*max_locals as usize) - locals.len()) {
                                                                                locals.push(RuntimeValue::Int(0));
                                                                            }
                                                                        }

                                                                        let mut newFrame = createFrame(info)?;
                                                                        newFrame.locals = locals;
                                                                        return do_execute_method(&info, constant_pool, &mut newFrame, jvm);
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
                                                                            JVMMethod::Bytecode(info) => {
                                                                                debug!("invoke bytecode method '{}'", name);
                                                                                let mut newFrame = createFrame(info)?;
                                                                                newFrame.locals = locals;
                                                                                do_execute_method(&info, constant_pool, &mut newFrame, jvm)?;
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
                                                                            JVMMethod::Bytecode(info) => {
                                                                                debug!("invoke bytecode method '{}'", name);
                                                                                let mut newFrame = createFrame(info)?;

                                                                                // fill in the rest of the locals array with 0
                                                                                if let Some(AttributeKind::Code { max_stack, max_locals, code, exception_table, attributes }) = lookup_code_attribute(info) {
                                                                                    for _i in 0..((*max_locals as usize) - locals.len()) {
                                                                                        locals.push(RuntimeValue::Int(0));
                                                                                    }
                                                                                }

                                                                                newFrame.locals = locals;
                                                                                return do_execute_method(&info, constant_pool, &mut newFrame, jvm)
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
            AttributeKind::Code { max_stack, max_locals, code, exception_table, attributes } => {
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
            if i1 >= i2 {
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
            match code[pc] {
                Opcodes::IConst0 => {
                    frame.push_value(RuntimeValue::Int(0));
                    pc += 1;
                },
                Opcodes::IConst1 => {
                    frame.push_value(RuntimeValue::Int(1));
                    pc += 1;
                },
                Opcodes::IConst2 => {
                    frame.push_value(RuntimeValue::Int(2));
                    pc += 1;
                },
                Opcodes::IConst3 => {
                    frame.push_value(RuntimeValue::Int(3));
                    pc += 1;
                },
                Opcodes::IConst4 => {
                    frame.push_value(RuntimeValue::Int(4));
                    pc += 1;
                },
                Opcodes::IConst5 => {
                    frame.push_value(RuntimeValue::Int(5));
                    pc += 1;
                },
                Opcodes::PushByte => {
                    let value = code[pc + 1] as i64;
                    frame.push_value(RuntimeValue::Int(value));
                    pc += 2;
                },
                Opcodes::IReturn => {
                    pc += 1;

                    // let value = frame.pop_value_force()?; 
                    // println!("returning value {:?}", value);

                    return Ok(frame.pop_value_force()?);
                    // return Ok(value);
                },
                Opcodes::Dup => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.push_value(value.clone());
                    frame.push_value(value);
                },
                Opcodes::AStore1 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[1] = value;
                },
                Opcodes::IStore => {
                    let index = code[pc + 1] as usize;
                    let value = frame.pop_value_force()?;
                    frame.locals[index] = value;
                    pc += 2;
                },
                Opcodes::IStore0 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[0] = value;
                },
                Opcodes::IStore1 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[1] = value;
                },
                Opcodes::IStore2 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[2] = value;
                },
                Opcodes::IStore3 => {
                    pc += 1;
                    let value = frame.pop_value_force()?;
                    frame.locals[3] = value;
                },
                Opcodes::ALoad0 => {
                    pc += 1;
                    let value = frame.locals[0].clone();
                    frame.push_value(value);
                },
                Opcodes::ALoad1 => {
                    pc += 1;
                    let value = frame.locals[1].clone();
                    frame.push_value(value);
                },
                Opcodes::TableSwitch => {
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
                Opcodes::IfICompareGreaterEqual => {
                    pc = do_icompare(frame, pc, make_int16(code[pc+1], code[pc+2]) as i16, |i1, i2| i1 >= i2)?;
                },
                Opcodes::Goto => {
                    let old = pc;
                    let offset = make_int16(code[pc+1], code[pc+2]);
                    pc = (pc as i16 + offset as i16) as usize;
                },
                Opcodes::ILoad => {
                    let index = code[pc + 1] as usize;
                    let value = frame.locals[index].clone();
                    frame.push_value(value);
                    pc += 2;
                },
                Opcodes::ILoad0 => {
                    pc += 1;
                    let value = frame.locals[0].clone();
                    debug!("  load0: loading value {:?}", value);
                    frame.push_value(value);
                },
                Opcodes::ILoad1 => {
                    pc += 1;
                    let value = frame.locals[1].clone();
                    frame.push_value(value);
                },
                Opcodes::ILoad2 => {
                    pc += 1;
                    let value = frame.locals[2].clone();
                    frame.push_value(value);
                },
                Opcodes::ILoad3 => {
                    pc += 1;
                    let value = frame.locals[3].clone();
                    frame.push_value(value);
                },
                Opcodes::IAdd => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 + i2)?;
                    frame.stack.push(value);
                },
                Opcodes::IMul => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 * i2)?;
                    frame.stack.push(value);
                },
                Opcodes::IDiv => {
                    pc += 1;
                    let value = do_iop(frame, |i1,i2| i1 / i2)?;
                    frame.stack.push(value);
                },
                Opcodes::IInc => {
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
                Opcodes::New => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    frame.push_value(create_new_object(constant_pool, jvm, total)?);

                    pc += 3;
                },
                Opcodes::GetField => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;
                    let objectref = frame.pop_value_force()?;
                    frame.push_value(getfield(constant_pool, jvm, total, objectref)?);
                    pc += 3;
                },
                Opcodes::PutField => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    let value = frame.pop_value_force()?;
                    let objectref = frame.pop_value_force()?;

                    putfield(constant_pool, jvm, total, objectref, value)?;

                    pc += 3;
                },
                Opcodes::InvokeSpecial => {
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;
                    invoke_special(constant_pool, frame, jvm, total)?;
                    pc += 3;
                },
                Opcodes::InvokeStatic => {
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
                Opcodes::GetStatic => {
                    debug!("Get static");
                    let b1 = code[pc+1] as usize;
                    let b2 = code[pc+2] as usize;
                    let total = (b1 << 8) | b2;

                    pc += 2;

                    op_getstatic(constant_pool, frame, jvm, total)?;

                    pc += 1;
                },
                Opcodes::InvokeVirtual => {
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
                Opcodes::Return => {
                    return Ok(RuntimeValue::Void);
                },
                Opcodes::PushRuntimeConstant => {
                    let index = code[pc+1] as usize;
                    push_runtime_constant(constant_pool, frame, index)?;
                    pc += 2;
                },
                _ => {
                    return Err(format!("Unknown opcode pc={} opcode=0x{:x}", pc, code[pc]));
                }
            }
        }

    } else {
        return Err("no code attribute".to_string());
    }

    return Ok(RuntimeValue::Void);
}

fn createStdoutObject() -> rc::Rc<cell::RefCell<JVMObject>> {
    return rc::Rc::new(cell::RefCell::new(JVMObject{
        class: "java/io/PrintStream".to_string(),
        fields: HashMap::new()
    }));
}

fn createJavaIoPrintStream<'a>() -> JVMClass<'a> {
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
        methods: methods,
        fields: fields,
    }
}

fn createJavaLangSystem<'a>() -> JVMClass<'a> {
    let mut fields = HashMap::new();

    fields.insert("out".to_string(), RuntimeValue::Object(createStdoutObject()));

    let mut methods = HashMap::new();

    return JVMClass{
        class: "java/lang/System".to_string(),
        methods: methods,
        fields: fields
    };
}

fn createFrame(method: &MethodInfo) -> Result<Frame, String> {
    if let Some(AttributeKind::Code { max_stack, max_locals, code, exception_table, attributes }) = lookup_code_attribute(method) {
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

fn createJavaLangObject<'a>() -> JVMClass<'a> {
    let mut fields = HashMap::new();
    let mut methods = HashMap::new();

    methods.insert("<init>".to_string(), JVMMethod::Native(|_args: &[RuntimeValue]| {
        return RuntimeValue::Void;
    }));

    return JVMClass{
        class: "java/lang/Object".to_string(),
        methods: methods,
        fields: fields,
    };
}

fn createRuntimeConst<'a>() -> RuntimeConst<'a> {
    let mut classes = HashMap::new();

    classes.insert("java/lang/System".to_string(), createJavaLangSystem());
    classes.insert("java/io/PrintStream".to_string(), createJavaIoPrintStream());
    classes.insert("java/lang/Object".to_string(), createJavaLangObject());

    return RuntimeConst{
        classes: classes,
    }
}

fn execute_method(jvm: &JVMClassFile, name: &str) -> Result<RuntimeValue, String> {
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

                    let mut frame = createFrame(&jvm.methods[i])?;

                    let mut runtime = createRuntimeConst();
                    runtime.add_class(createJvmClass(jvm)?);

                    return do_execute_method(&jvm.methods[i], &jvm.constant_pool, &mut frame, &runtime);
                }
            },
            None => {
                return Err("Error: method name index is invalid".to_string());
            }
        }
    }

    return Err("no such method found".to_string());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // iterate through arguments and print each one out
    /*
    for arg in args.iter() {
        println!("{}", arg);
    }
    */
    // print just the first argument out, but only if there is at least one argument
    if args.len() > 1 {
        match parse_class_file(args[1].as_str()) {
            Ok(class_file) => {
                match execute_method(&class_file, "main") {
                    Ok(_) => {
                    },
                    Err(err) => {
                        println!("Error: {0}", err);
                    }
                }
            },
            Err(err) => {
                println!("Error: {0}", err);
            }
        }
    }
}
