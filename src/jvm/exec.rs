use std::collections::HashMap;
use std::rc;
use std::cell;
use std::fmt;

use crate::debug;
use super::data::*;

mod dynamic;
mod arrays;

#[cfg(test)]
mod tests;

// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5
pub mod opcodes {
    pub const LCONST0:u8 = 0x09; // lconst_0
    pub const LCONST1:u8 = 0x0a; // lconst_1
    pub const LLOAD:u8 = 0x16; // lload
    pub const LLOAD0:u8 = 0x1e; // lload_0
    pub const LLOAD1:u8 = 0x1f; // lload_1
    pub const LLOAD2:u8 = 0x20; // lload_2
    pub const LLOAD3:u8 = 0x21; // lload_3
    pub const LSTORE:u8 = 0x37; // lstore
    pub const LSTORE0:u8 = 0x3f; // lstore_0
    pub const LSTORE1:u8 = 0x40; // lstore_1
    pub const LSTORE2:u8 = 0x41; // lstore_2
    pub const LSTORE3:u8 = 0x42; // lstore_3
    pub const LALOAD:u8 = 0x2f; // laload
    pub const LASTORE:u8 = 0x50; // lastore
    pub const LADD:u8 = 0x61; // ladd
    pub const LSUB:u8 = 0x65; // lsub
    pub const LREM:u8 = 0x71; // lrem
    pub const LSHL:u8 = 0x79; // lshl
    pub const LSHR:u8 = 0x7b; // lshr
    pub const LUSHR:u8 = 0x7d; // lushr
    pub const LXOR:u8 = 0x83; // lxor
    pub const LAND:u8 = 0x7f; // land
    pub const L2I:u8 = 0x88; // l2i
    pub const L2F:u8 = 0x89; // l2f
    pub const L2D:u8 = 0x8a; // l2d
    pub const LCMP:u8 = 0x94; // lcmp
    pub const ICONSTM1:u8 = 0x02; // iconst_m1
    pub const ICONST0:u8 = 0x3; // iconst_0
    pub const ICONST1:u8 = 0x4; // iconst_1
    pub const ICONST2:u8 = 0x5; // iconst_2
    pub const ICONST3:u8 = 0x6; // iconst_3
    pub const ICONST4:u8 = 0x7; // iconst_4
    pub const ICONST5:u8 = 0x8; // iconst_5
    pub const FCONST0:u8 = 0x0b; // fconst_0
    pub const FCONST1:u8 = 0x0c; // fconst_1
    pub const FCONST2:u8 = 0x0d; // fconst_2
    pub const FLOAD:u8 = 0x17; // fload
    pub const FLOAD0:u8 = 0x22; // fload_0
    pub const FLOAD1:u8 = 0x23; // fload_1
    pub const FLOAD2:u8 = 0x24; // fload_2
    pub const FLOAD3:u8 = 0x25; // fload_3
    pub const FSTORE:u8 = 0x38; // fstore
    pub const FSTORE0:u8 = 0x43; // fstore_0
    pub const FSTORE1:u8 = 0x44; // fstore_1
    pub const FSTORE2:u8 = 0x45; // fstore_2
    pub const FSTORE3:u8 = 0x46; // fstore_3
    pub const FADD:u8 = 0x62; // fadd
    pub const FSUB:u8 = 0x66; // fsub
    pub const FMUL:u8 = 0x6a; // fmul
    pub const FDIV:u8 = 0x6e; // fdiv
    pub const FREM:u8 = 0x72; // frem
    pub const FNEG:u8 = 0x76; // fneg
    pub const F2I:u8 = 0x8b; // f2i
    pub const F2D:u8 = 0x8d; // f2d
    pub const DCONST0:u8 = 0x0e; // dconst_0
    pub const DCONST1:u8 = 0x0f; // dconst_1
    pub const DLOAD:u8 = 0x18; // dload
    pub const DLOAD0:u8 = 0x26; // dload_0
    pub const DLOAD1:u8 = 0x27; // dload_1
    pub const DLOAD2:u8 = 0x28; // dload_2
    pub const DLOAD3:u8 = 0x29; // dload_3
    pub const DALOAD:u8 = 0x31; // daload
    pub const DSTORE:u8 = 0x39; // dstore
    pub const DSTORE0:u8 = 0x47; // dstore_0
    pub const DSTORE1:u8 = 0x48; // dstore_1
    pub const DSTORE2:u8 = 0x49; // dstore_2
    pub const DSTORE3:u8 = 0x4a; // dstore_3
    pub const DASTORE:u8 = 0x52; // dastore
    pub const DADD:u8 = 0x63; // dadd
    pub const DSUB:u8 = 0x67; // dsub
    pub const DMUL:u8 = 0x6b; // dmul
    pub const DDIV:u8 = 0x6f; // ddiv
    pub const DREM:u8 = 0x73; // drem
    pub const DNEG:u8 = 0x77; // dneg
    pub const FCMPL:u8 = 0x95; // fcmpl
    pub const FCMPG:u8 = 0x96; // fcmpg
    pub const DCMPL:u8 = 0x97; // dcmpl
    pub const DCMPG:u8 = 0x98; // dcmpg
    pub const IFEQ:u8 = 0x99; // ifeq
    pub const IFNE:u8 = 0x9a; // ifne
    pub const IFLT:u8 = 0x9b; // iflt
    pub const IFGT:u8 = 0x9d; // ifgt
    pub const IFNULL:u8 = 0xc6; // ifnull
    pub const IFNONNULL:u8 = 0xc7; // ifnonnull
    pub const IFGE:u8 = 0x9c; // ifge
    pub const IFLE:u8 = 0x9e; // ifle
    pub const I2L:u8 = 0x85; // i2l
    pub const I2F:u8 = 0x86; // i2f
    pub const I2D:u8 = 0x87; // i2d
    pub const I2B:u8 = 0x91; // i2b
    pub const I2C:u8 = 0x92; // i2c
    pub const I2S:u8 = 0x93; // i2s
    pub const D2I:u8 = 0x8e; // d2i
    pub const D2L:u8 = 0x8f; // d2l
    pub const D2F:u8 = 0x90; // d2f
    pub const ACONSTNULL:u8 = 0x01; // aconst_null
    pub const FALOAD:u8 = 0x30; // faload
    pub const FASTORE:u8 = 0x51; // fastore
    pub const CALOAD:u8 = 0x34; // caload
    pub const CASTORE:u8 = 0x55; // castore
    pub const BALOAD:u8 = 0x33; // baload
    pub const BASTORE:u8 = 0x54; // bastore
    pub const AALOAD:u8 = 0x32; // aaload
    pub const AASTORE:u8 = 0x53; // aastore
    pub const ANEWARRAY:u8 = 0xbd; // anewarray
    pub const NEWARRAY:u8 = 0xbc; // newarray
    pub const MULTIANEWARRAY:u8 = 0xc5; // multianewarray
    pub const IALOAD:u8 = 0x2e; // iaload
    pub const IASTORE:u8 = 0x4f; // iastore
    pub const ARRAYLENGTH:u8 = 0xbe; // arraylength
    pub const PUSHBYTE:u8 = 0x10; // bipush
    pub const SIPUSH:u8 = 0x11; // sipush
    pub const SALOAD:u8 = 0x35; // saload
    pub const SASTORE:u8 = 0x56; // sastore
    pub const PUSHRUNTIMECONSTANT:u8 = 0x12; // ldc
    pub const ILOAD:u8 = 0x15; // iload
    pub const ILOAD0:u8 = 0x1a; // iload_0
    pub const ILOAD1:u8 = 0x1b; // iload_1
    pub const ILOAD2:u8 = 0x1c; // iload_2
    pub const ILOAD3:u8 = 0x1d; // iload_3
    pub const ALOAD0:u8 = 0x2a; // aload_0
    pub const ALOAD1:u8 = 0x2b; // aload_1
    pub const ALOAD2:u8 = 0x2c; // aload_2
    pub const ALOAD3:u8 = 0x2d; // aload_3
    pub const ISTORE:u8 = 0x36; // istore
    pub const ISTORE0:u8 = 0x3b; // istore_0
    pub const ISTORE1:u8 = 0x3c; // istore_1
    pub const ISTORE2:u8 = 0x3d; // istore_2
    pub const ISTORE3:u8 = 0x3e; // istore_3
    pub const ASTORE0:u8 = 0x4b; // astore_0
    pub const ASTORE1:u8 = 0x4c; // astore_1
    pub const ASTORE2:u8 = 0x4d; // astore_2
    pub const ASTORE3:u8 = 0x4e; // astore_3
    pub const DUP:u8 = 0x59; // dup
    pub const NOP:u8 = 0x00; // nop
    pub const POP:u8 = 0x57; // pop
    pub const POP2:u8 = 0x58; // pop2
    pub const SWAP:u8 = 0x5f; // swap
    pub const DUP2X1:u8 = 0x5d; // dup2_x1
    pub const DUP2X2:u8 = 0x5e; // dup2_x2
    pub const IADD:u8 = 0x60; // iadd
    pub const IMUL:u8 = 0x68; // imul
    pub const INEG:u8 = 0x74; // ineg
    pub const ISHL:u8 = 0x78; // ishl
    pub const ISHR:u8 = 0x7a; // ishr
    pub const IUSHR:u8 = 0x7c; // iushr
    pub const IAND:u8 = 0x7e; // iand
    pub const IOR:u8 = 0x80; // ior
    pub const IXOR:u8 = 0x82; // ixor
    pub const IDIV:u8 = 0x6c; // idiv
    pub const IINC:u8 = 0x84; // iinc
    pub const WIDE:u8 = 0xc4; // wide
    pub const TABLESWITCH:u8 = 0xaa; // tableswitch
    pub const LOOKUPSWITCH:u8 = 0xab; // lookupswitch
    pub const IFICOMPAREEQUAL:u8 = 0x9f; // if_icmpeq
    pub const IFICOMPARENOTEQUAL:u8 = 0xa0; // if_icmpne
    pub const IFICOMPAREGREATER:u8 = 0xa3; // if_icmpgt
    pub const IFICOMPARELESSEQUAL:u8 = 0xa4; // if_icmple
    pub const IFACMPEQ:u8 = 0xa5; // if_acmpeq
    pub const IFACMPNE:u8 = 0xa6; // if_acmpne
    pub const IFICOMPARELESS:u8 = 0xa1; // if_icmplt
    pub const IFICOMPAREGREATEREQUAL:u8 = 0xa2; // if_icmpge
    pub const GOTO:u8 = 0xa7; // goto
    pub const JSR:u8 = 0xa8; // jsr
    pub const RET:u8 = 0xa9; // ret
    pub const GOTOW:u8 = 0xc8; // goto_w
    pub const IRETURN:u8 = 0xac; // ireturn
    pub const DRETURN:u8 = 0xaf; // dreturn
    pub const ARETURN:u8 = 0xb0; // areturn
    pub const RETURN:u8 = 0xb1; // return
    pub const GETSTATIC:u8 = 0xb2; // getstatic
    pub const PUTSTATIC:u8 = 0xb3; // putstatic
    pub const GETFIELD:u8 = 0xb4; // getfield
    pub const PUTFIELD:u8 = 0xb5; // putfield
    pub const INVOKEVIRTUAL:u8 = 0xb6; // invokevirtual
    pub const INVOKESPECIAL:u8 = 0xb7; // invokespecial
    pub const INVOKESTATIC:u8 = 0xb8; // invokestatic
    pub const INVOKEDYNAMIC:u8 = 0xba; // invokedynamic
    pub const NEW:u8 = 0xbb; // new
    pub const ATHROW:u8 = 0xbf; // athrow
    pub const CHECKCAST:u8 = 0xc0; // checkcast
    pub const INSTANCEOF:u8 = 0xc1; // instanceof
    pub const MONITORENTER:u8 = 0xc2; // monitorenter
    pub const MONITOREXIT:u8 = 0xc3; // monitorexit
}

mod array_types {
    pub const BOOLEAN: u8 = 4;
    pub const CHAR: u8 = 5;
    pub const FLOAT: u8 = 6;
    pub const INT: u8 = 10;
    pub const SHORT: u8 = 9;
    pub const LONG: u8 = 11;
    pub const DOUBLE: u8 = 7;
    pub const BYTE: u8 = 8;

    pub fn is_supported(atype: u8) -> bool {
        matches!(atype, BOOLEAN | CHAR | FLOAT | DOUBLE | BYTE | SHORT | LONG | INT)
    }
}

#[derive(Clone)]
pub enum RuntimeValue{
    ReturnAddress(usize),
    Int(i64),
    Long(i64),
    Float(f32),
    Double(f64),
    DoubleArray(rc::Rc<cell::RefCell<Vec<f64>>>),
    CharArray(rc::Rc<cell::RefCell<Vec<u16>>>),
    FloatArray(rc::Rc<cell::RefCell<Vec<f32>>>),
    LongArray(rc::Rc<cell::RefCell<Vec<i64>>>),
    ShortArray(rc::Rc<cell::RefCell<Vec<i16>>>),
    IntArray(rc::Rc<cell::RefCell<Vec<i32>>>),
    ByteArray(rc::Rc<cell::RefCell<JVMByteArray>>),
    ReferenceArray(rc::Rc<cell::RefCell<JVMReferenceArray>>),
    Null,
    Void,
    String(rc::Rc<String>),
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
            RuntimeValue::IntArray(values) => {
                write!(f, "IntArray({:?})", values.borrow())
            },
            RuntimeValue::ShortArray(values) => {
                write!(f, "ShortArray({:?})", values.borrow())
            },
            RuntimeValue::LongArray(values) => {
                write!(f, "LongArray({:?})", values.borrow())
            },
            RuntimeValue::FloatArray(values) => {
                write!(f, "FloatArray({:?})", values.borrow())
            },
            RuntimeValue::CharArray(values) => {
                write!(f, "CharArray({:?})", values.borrow())
            },
            RuntimeValue::ByteArray(array) => {
                write!(f, "ByteArray({:?})", array.borrow().values)
            },
            RuntimeValue::ReferenceArray(array) => {
                let array = array.borrow();
                write!(f, "ReferenceArray({}, length={})", array.component_class, array.values.len())
            },
            RuntimeValue::ReturnAddress(address) => write!(f, "ReturnAddress({})", address),
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
                        source: Some(jvmclass),
                        class: class_name.to_string(),
                        super_class: if jvmclass.super_class == 0 { None } else {
                            Some(lookup_class_name(&jvmclass.constant_pool, jvmclass.super_class as usize)?.to_string())
                        },
                        methods: methods,
                        fields: cell::RefCell::new(HashMap::new()),
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
    source: Option<&'a JVMClassFile>,
    class: String,
    super_class: Option<String>,
    methods: HashMap<String, JVMMethod<'a>>,
    fields: cell::RefCell<HashMap<String, RuntimeValue>>,
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
    interned_strings: cell::RefCell<HashMap<String, rc::Rc<String>>>,
    // There is one execution thread; counts support reentrant locking.
    monitors: cell::RefCell<Vec<(RuntimeValue, usize)>>,
}

fn lookup_class_name(pool: &ConstantPool, index: usize) -> Result<&str, String> {
    if let Some(ConstantPoolEntry::Classref(name)) = constant_pool_lookup(pool, index) {
        if let Some(name) = lookup_utf8_constant(pool, *name as usize) {
            return Ok(name);
        }
    }
    Err(format!("invalid class reference {}", index))
}

fn execute_monitor(jvm: &RuntimeConst, value: RuntimeValue, enter: bool) -> Result<(), String> {
    // Validate reference operands before attempting an identity comparison.
    references_equal(&value, &value)?;
    let exception = if matches!(value, RuntimeValue::Null) {
        Some("java/lang/NullPointerException")
    } else {
        let mut monitors = jvm.monitors.borrow_mut();
        let index = monitors.iter().position(|(held, _)| references_equal(held, &value).unwrap_or(false));
        if enter {
            if let Some(index) = index {
                monitors[index].1 = monitors[index].1.checked_add(1).ok_or("monitor count overflow")?;
            } else {
                monitors.push((value, 1));
            }
            None
        } else if let Some(index) = index {
            monitors[index].1 -= 1;
            if monitors[index].1 == 0 {
                monitors.remove(index);
            }
            None
        } else {
            Some("java/lang/IllegalMonitorStateException")
        }
    };
    if let Some(name) = exception {
        let class = jvm.lookup_class(name).ok_or("monitor exception class not found")?;
        *jvm.pending_exception.borrow_mut() = Some(RuntimeValue::Object(
            rc::Rc::new(cell::RefCell::new(class.create_object()))
        ));
    }
    Ok(())
}

fn references_equal(left: &RuntimeValue, right: &RuntimeValue) -> Result<bool, String> {
    fn is_reference(value: &RuntimeValue) -> bool {
        matches!(value, RuntimeValue::Null | RuntimeValue::Object(_) | RuntimeValue::String(_)
            | RuntimeValue::ReferenceArray(_) | RuntimeValue::IntArray(_) | RuntimeValue::LongArray(_)
            | RuntimeValue::FloatArray(_) | RuntimeValue::DoubleArray(_) | RuntimeValue::ByteArray(_)
            | RuntimeValue::CharArray(_) | RuntimeValue::ShortArray(_))
    }
    if !is_reference(left) || !is_reference(right) {
        return Err("reference comparison requires references".to_string());
    }
    Ok(match (left, right) {
        (RuntimeValue::Null, RuntimeValue::Null) => true,
        (RuntimeValue::Object(left), RuntimeValue::Object(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::String(left), RuntimeValue::String(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::ReferenceArray(left), RuntimeValue::ReferenceArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::IntArray(left), RuntimeValue::IntArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::LongArray(left), RuntimeValue::LongArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::FloatArray(left), RuntimeValue::FloatArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::DoubleArray(left), RuntimeValue::DoubleArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::ByteArray(left), RuntimeValue::ByteArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::CharArray(left), RuntimeValue::CharArray(right)) => rc::Rc::ptr_eq(left, right),
        (RuntimeValue::ShortArray(left), RuntimeValue::ShortArray(right)) => rc::Rc::ptr_eq(left, right),
        _ => false,
    })
}

fn reference_assignable(jvm: &RuntimeConst, value: &RuntimeValue, target: &str) -> bool {
    match value {
        RuntimeValue::Null => true,
        RuntimeValue::Object(object) => is_instance_of(jvm, &object.borrow().class, target),
        RuntimeValue::String(_) => target == "java/lang/String" || target == "java/lang/Object",
        RuntimeValue::DoubleArray(_) => target == "[D" || array_supertype(target),
        RuntimeValue::CharArray(_) => target == "[C" || array_supertype(target),
        RuntimeValue::FloatArray(_) => target == "[F" || array_supertype(target),
        RuntimeValue::LongArray(_) => target == "[J" || array_supertype(target),
        RuntimeValue::ShortArray(_) => target == "[S" || array_supertype(target),
        RuntimeValue::IntArray(_) => target == "[I" || array_supertype(target),
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

fn op_putstatic(pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, index: usize) -> Result<(), String> {
    let (class_index, name_type) = match constant_pool_lookup(pool, index) {
        Some(ConstantPoolEntry::Fieldref { class_index, name_and_type_index }) => (*class_index, *name_and_type_index),
        _ => return Err("putstatic requires a field reference".to_string()),
    };
    let class_name = lookup_class_name(pool, class_index as usize)?;
    let name = match constant_pool_lookup(pool, name_type as usize) {
        Some(ConstantPoolEntry::NameAndType { name_index, .. }) => {
            lookup_utf8_constant(pool, *name_index as usize).ok_or("invalid static field name")?
        },
        _ => return Err("invalid static field name and type".to_string()),
    };
    let class = jvm.lookup_class(class_name).ok_or("static field class not found")?;
    let value = frame.pop_value_force()?;
    class.fields.borrow_mut().insert(name.to_string(), value);
    Ok(())
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
                                                    match class.fields.borrow().get(name) {
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

fn push_runtime_constant(constant_pool: &ConstantPool, frame: &mut Frame, jvm: &RuntimeConst, index: usize) -> Result<(), String> {
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
                        let value = jvm.interned_strings.borrow_mut().entry(name.clone())
                            .or_insert_with(|| rc::Rc::new(name.clone())).clone();
                        frame.push_value(RuntimeValue::String(value));
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

fn group_start(stack: &[RuntimeValue], mut end: usize, mut slots: usize) -> Result<usize, String> {
    while slots > 0 {
        end = end.checked_sub(1).ok_or("Stack underflow")?;
        let width = match &stack[end] {
            RuntimeValue::Double(_) | RuntimeValue::Long(_) => 2,
            RuntimeValue::Void => return Err("invalid void operand".to_string()),
            _ => 1,
        };
        slots = slots.checked_sub(width).ok_or("invalid stack categories")?;
    }
    Ok(end)
}

fn execute_wide(code: &[u8], pc: usize, frame: &mut Frame) -> Result<usize, String> {
    let operands = code.get(pc + 1..pc + 4).ok_or("truncated wide instruction")?;
    let index = make_int16(operands[1], operands[2]) as usize;
    match operands[0] {
        opcodes::ILOAD => {
            let value = match frame.locals.get(index) {
                Some(RuntimeValue::Int(value)) => *value,
                _ => return Err("wide iload requires an int local".to_string()),
            };
            frame.push_value(RuntimeValue::Int(value));
            Ok(pc + 4)
        },
        opcodes::ISTORE => {
            if index >= frame.locals.len() {
                return Err("wide istore local index out of bounds".to_string());
            }
            let value = frame.pop_value_force()?;
            if !matches!(value, RuntimeValue::Int(_)) {
                return Err("wide istore requires an int".to_string());
            }
            frame.locals[index] = value;
            Ok(pc + 4)
        },
        opcodes::IINC => {
            let increment = code.get(pc + 4..pc + 6).ok_or("truncated wide iinc")?;
            let increment = make_int16(increment[0], increment[1]) as i16 as i32;
            match frame.locals.get_mut(index) {
                Some(RuntimeValue::Int(value)) => {
                    *value = (*value as i32).wrapping_add(increment) as i64;
                },
                _ => return Err("wide iinc requires an int local".to_string()),
            }
            Ok(pc + 6)
        },
        _ => Err(format!("unsupported wide opcode 0x{:02x}", operands[0])),
    }
}

fn swap_values(frame: &mut Frame) -> Result<(), String> {
    // swap permits two category-1 values, never a long or double.
    let top = group_start(&frame.stack, frame.stack.len(), 1)?;
    let below = group_start(&frame.stack, top, 1)?;
    frame.stack.swap(top, below);
    Ok(())
}

fn pop_slots(frame: &mut Frame, slots: usize) -> Result<(), String> {
    let start = group_start(&frame.stack, frame.stack.len(), slots)?;
    frame.stack.truncate(start);
    Ok(())
}

fn duplicate_two_slots(frame: &mut Frame, depth: usize) -> Result<(), String> {
    // Category-2 values occupy one Vec entry, but two JVM stack slots.
    let top = group_start(&frame.stack, frame.stack.len(), 2)?;
    let insert = group_start(&frame.stack, top, depth)?;
    let duplicate = frame.stack[top..].to_vec();
    frame.stack.splice(insert..insert, duplicate);
    Ok(())
}

fn do_iop(frame: &mut Frame, op: fn(i64, i64) -> i64) -> Result<RuntimeValue, String> {
    // The right operand is on top of the JVM stack.
    let value2 = frame.pop_value_force()?;
    let value1 = frame.pop_value_force()?;
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
                return Ok((pc as isize + offset as isize) as usize);
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

fn lookup_switch_target(code: &[u8], pc: usize, key: i32) -> Result<usize, String> {
    // Padding aligns the default field relative to the start of the method.
    let start = (pc + 4) & !3;
    let header = code.get(start..start + 8).ok_or("truncated lookupswitch header")?;
    let mut offset = make_int32(header[0], header[1], header[2], header[3]) as i32;
    let count = make_int32(header[4], header[5], header[6], header[7]) as i32;
    if count < 0 {
        return Err("negative lookupswitch pair count".to_string());
    }
    let bytes = (count as usize).checked_mul(8).ok_or("lookupswitch table too large")?;
    let end = (start + 8).checked_add(bytes).ok_or("lookupswitch table too large")?;
    let pairs = code.get(start + 8..end).ok_or("truncated lookupswitch pairs")?;
    let mut previous = None;
    for pair in pairs.chunks_exact(8) {
        let candidate = make_int32(pair[0], pair[1], pair[2], pair[3]) as i32;
        if previous.map_or(false, |previous| candidate <= previous) {
            return Err("lookupswitch keys must be strictly increasing".to_string());
        }
        previous = Some(candidate);
        if candidate == key {
            offset = make_int32(pair[4], pair[5], pair[6], pair[7]) as i32;
        }
    }
    let target = pc as i64 + offset as i64;
    if target < 0 || target >= code.len() as i64 {
        return Err("lookupswitch target out of bounds".to_string());
    }
    Ok(target as usize)
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
                opcodes::MULTIANEWARRAY => {
                    let operands = code.get(pc + 1..pc + 4).ok_or("truncated multianewarray")?;
                    let index = make_int16(operands[0], operands[1]) as usize;
                    let descriptor = lookup_class_name(constant_pool, index)?;
                    let dimensions = operands[2] as usize;
                    if dimensions == 0 || dimensions > descriptor.bytes().take_while(|c| *c == b'[').count() {
                        return Err("invalid multianewarray dimensions".to_string());
                    }
                    let mut counts = Vec::with_capacity(dimensions);
                    for _ in 0..dimensions {
                        match frame.pop_value_force()? {
                            RuntimeValue::Int(count) if count >= 0 => counts.push(count as usize),
                            _ => return Err("invalid array length".to_string()),
                        }
                    }
                    counts.reverse();
                    frame.push_value(arrays::allocate_multidimensional(descriptor, &counts)?);
                    pc += 4;
                },
                opcodes::IALOAD | opcodes::IASTORE => {
                    let stored = if code[pc] == opcodes::IASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Int(value) => Some(value as i32),
                            _ => return Err("iastore requires an integer".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::IntArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(RuntimeValue::Int(*slot as i64));
                            }
                        },
                        _ => return Err("int array required".to_string()),
                    }
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
                    } else if atype == array_types::INT {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0);
                        frame.push_value(RuntimeValue::IntArray(rc::Rc::new(cell::RefCell::new(values))));
                    } else if atype == array_types::SHORT {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0);
                        frame.push_value(RuntimeValue::ShortArray(rc::Rc::new(cell::RefCell::new(values))));
                    } else if atype == array_types::LONG {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0);
                        frame.push_value(RuntimeValue::LongArray(rc::Rc::new(cell::RefCell::new(values))));
                    } else if atype == array_types::FLOAT {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0.0);
                        frame.push_value(RuntimeValue::FloatArray(rc::Rc::new(cell::RefCell::new(values))));
                    } else if atype == array_types::CHAR {
                        let mut values = Vec::new();
                        values.try_reserve_exact(count).map_err(|err| err.to_string())?;
                        values.resize(count, 0);
                        frame.push_value(RuntimeValue::CharArray(rc::Rc::new(cell::RefCell::new(values))));
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
                        RuntimeValue::CharArray(values) => values.borrow().len(),
                        RuntimeValue::FloatArray(values) => values.borrow().len(),
                        RuntimeValue::LongArray(values) => values.borrow().len(),
                        RuntimeValue::ShortArray(values) => values.borrow().len(),
                        RuntimeValue::IntArray(values) => values.borrow().len(),
                        RuntimeValue::ByteArray(array) => array.borrow().values.len(),
                        RuntimeValue::ReferenceArray(array) => array.borrow().values.len(),
                        _ => return Err("arraylength requires an array".to_string()),
                    };
                    frame.push_value(RuntimeValue::Int(length as i64));
                    pc += 1;
                },
                opcodes::SALOAD | opcodes::SASTORE => {
                    let stored = if code[pc] == opcodes::SASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Int(value) => Some(value as i16),
                            _ => return Err("sastore requires an integer".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::ShortArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(RuntimeValue::Int(*slot as i64));
                            }
                        },
                        _ => return Err("short array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::LALOAD | opcodes::LASTORE => {
                    let stored = if code[pc] == opcodes::LASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Long(value) => Some(value),
                            _ => return Err("lastore requires a long".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::LongArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(RuntimeValue::Long(*slot));
                            }
                        },
                        _ => return Err("long array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::LCONST0 | opcodes::LCONST1 => {
                    frame.push_value(RuntimeValue::Long((code[pc] - opcodes::LCONST0) as i64));
                    pc += 1;
                },
                opcodes::LLOAD | opcodes::LLOAD0..=opcodes::LLOAD3 => {
                    let (index, size) = if code[pc] == opcodes::LLOAD {
                        (*code.get(pc + 1).ok_or("truncated lload")? as usize, 2)
                    } else { ((code[pc] - opcodes::LLOAD0) as usize, 1) };
                    if index + 1 >= frame.locals.len() {
                        return Err("invalid lload local index".to_string());
                    }
                    match frame.locals.get(index) {
                        Some(RuntimeValue::Long(value)) => frame.push_value(RuntimeValue::Long(*value)),
                        _ => return Err("lload requires a long local".to_string()),
                    }
                    pc += size;
                },
                opcodes::LSTORE | opcodes::LSTORE0..=opcodes::LSTORE3 => {
                    let (index, size) = if code[pc] == opcodes::LSTORE {
                        (*code.get(pc + 1).ok_or("truncated lstore")? as usize, 2)
                    } else { ((code[pc] - opcodes::LSTORE0) as usize, 1) };
                    if index + 1 >= frame.locals.len() {
                        return Err("invalid lstore local index".to_string());
                    }
                    let value = frame.pop_value_force()?;
                    if !matches!(value, RuntimeValue::Long(_)) {
                        return Err("lstore requires a long".to_string());
                    }
                    frame.locals[index] = value;
                    frame.locals[index + 1] = RuntimeValue::Void;
                    pc += size;
                },
                opcodes::L2D | opcodes::L2F | opcodes::L2I => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Long(value) => value,
                        _ => return Err("long conversion requires a long".to_string()),
                    };
                    let converted = match code[pc] {
                        opcodes::L2D => RuntimeValue::Double(value as f64),
                        opcodes::L2F => RuntimeValue::Float(value as f32),
                        _ => RuntimeValue::Int(value as i32 as i64),
                    };
                    frame.push_value(converted);
                    pc += 1;
                },
                opcodes::LSHL | opcodes::LSHR | opcodes::LUSHR => {
                    let distance = match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => value as u32 & 0x3f,
                        _ => return Err("long shift requires an int distance".to_string()),
                    };
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Long(value) => value,
                        _ => return Err("long shift requires a long value".to_string()),
                    };
                    let shifted = match code[pc] {
                        opcodes::LSHL => value << distance,
                        opcodes::LSHR => value >> distance,
                        _ => ((value as u64) >> distance) as i64,
                    };
                    frame.push_value(RuntimeValue::Long(shifted));
                    pc += 1;
                },
                opcodes::LREM => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Long(_), RuntimeValue::Long(0)) => {
                            let class = jvm.lookup_class("java/lang/ArithmeticException")
                                .ok_or("ArithmeticException class not found")?;
                            *jvm.pending_exception.borrow_mut() = Some(RuntimeValue::Object(
                                rc::Rc::new(cell::RefCell::new(class.create_object()))
                            ));
                        },
                        (RuntimeValue::Long(left), RuntimeValue::Long(right)) => {
                            // MIN_VALUE % -1 is zero in Java, not an overflow panic.
                            frame.push_value(RuntimeValue::Long(left.wrapping_rem(right)));
                        },
                        _ => return Err("lrem requires longs".to_string()),
                    }
                    pc += 1;
                },
                opcodes::LADD | opcodes::LSUB | opcodes::LAND | opcodes::LXOR | opcodes::LCMP => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    let result = match (left, right) {
                        (RuntimeValue::Long(left), RuntimeValue::Long(right)) => match code[pc] {
                            opcodes::LADD => RuntimeValue::Long(left.wrapping_add(right)),
                            opcodes::LSUB => RuntimeValue::Long(left.wrapping_sub(right)),
                            opcodes::LXOR => RuntimeValue::Long(left ^ right),
                            opcodes::LAND => RuntimeValue::Long(left & right),
                            _ => RuntimeValue::Int(match left.cmp(&right) {
                                std::cmp::Ordering::Less => -1,
                                std::cmp::Ordering::Equal => 0,
                                std::cmp::Ordering::Greater => 1,
                            }),
                        },
                        _ => return Err("long operation requires longs".to_string()),
                    };
                    frame.push_value(result);
                    pc += 1;
                },
                opcodes::FALOAD | opcodes::FASTORE => {
                    let stored = if code[pc] == opcodes::FASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Float(value) => Some(value),
                            _ => return Err("fastore requires a float".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::FloatArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                frame.push_value(RuntimeValue::Float(*slot));
                            }
                        },
                        _ => return Err("float array required".to_string()),
                    }
                    pc += 1;
                },
                opcodes::CALOAD | opcodes::CASTORE => {
                    let stored = if code[pc] == opcodes::CASTORE {
                        match frame.pop_value_force()? {
                            RuntimeValue::Int(value) => Some(value as u16),
                            _ => return Err("castore requires an integer".to_string()),
                        }
                    } else { None };
                    let index = match frame.pop_value_force()? {
                        RuntimeValue::Int(index) if index >= 0 => index as usize,
                        RuntimeValue::Int(_) => return Err("array index out of bounds".to_string()),
                        _ => return Err("array index must be an integer".to_string()),
                    };
                    match frame.pop_value_force()? {
                        RuntimeValue::CharArray(values) => {
                            let mut values = values.borrow_mut();
                            let slot = values.get_mut(index).ok_or("array index out of bounds")?;
                            if let Some(value) = stored {
                                *slot = value;
                            } else {
                                // Java chars are unsigned 16-bit values, not Unicode scalar values.
                                frame.push_value(RuntimeValue::Int(*slot as i64));
                            }
                        },
                        _ => return Err("char array required".to_string()),
                    }
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
                opcodes::FCONST0 | opcodes::FCONST1 | opcodes::FCONST2 => {
                    frame.push_value(RuntimeValue::Float((code[pc] - opcodes::FCONST0) as f32));
                    pc += 1;
                },
                opcodes::FLOAD | opcodes::FLOAD0..=opcodes::FLOAD3 => {
                    let (index, size) = if code[pc] == opcodes::FLOAD {
                        (code[pc + 1] as usize, 2)
                    } else { ((code[pc] - opcodes::FLOAD0) as usize, 1) };
                    match frame.locals.get(index) {
                        Some(RuntimeValue::Float(value)) => frame.push_value(RuntimeValue::Float(*value)),
                        _ => return Err("fload requires a float local".to_string()),
                    }
                    pc += size;
                },
                opcodes::FSTORE | opcodes::FSTORE0..=opcodes::FSTORE3 => {
                    let (index, size) = if code[pc] == opcodes::FSTORE {
                        (code[pc + 1] as usize, 2)
                    } else { ((code[pc] - opcodes::FSTORE0) as usize, 1) };
                    let value = frame.pop_value_force()?;
                    if !matches!(value, RuntimeValue::Float(_)) {
                        return Err("fstore requires a float".to_string());
                    }
                    *frame.locals.get_mut(index).ok_or("invalid fstore local index")? = value;
                    pc += size;
                },
                opcodes::F2I | opcodes::F2D => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Float(value) => value,
                        _ => return Err("float conversion requires a float".to_string()),
                    };
                    let converted = if code[pc] == opcodes::F2I {
                        // Truncate toward zero, saturate overflow, and map NaN to zero.
                        RuntimeValue::Int(value as i32 as i64)
                    } else {
                        RuntimeValue::Double(value as f64)
                    };
                    frame.push_value(converted);
                    pc += 1;
                },
                opcodes::FADD | opcodes::FSUB | opcodes::FMUL | opcodes::FDIV | opcodes::FREM => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Float(left), RuntimeValue::Float(right)) => {
                            let value = match code[pc] {
                                opcodes::FADD => left + right,
                                opcodes::FSUB => left - right,
                                opcodes::FMUL => left * right,
                                opcodes::FDIV => left / right,
                                _ => left % right,
                            };
                            frame.push_value(RuntimeValue::Float(value));
                        },
                        _ => return Err("float arithmetic requires floats".to_string()),
                    }
                    pc += 1;
                },
                opcodes::FNEG => {
                    match frame.pop_value_force()? {
                        RuntimeValue::Float(value) => frame.push_value(RuntimeValue::Float(-value)),
                        _ => return Err("fneg requires a float".to_string()),
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
                opcodes::D2I | opcodes::D2L | opcodes::D2F => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Double(value) => value,
                        _ => return Err("double conversion requires a double".to_string()),
                    };
                    let converted = match code[pc] {
                        // Rust casts truncate toward zero, saturate overflow, and map NaN to zero.
                        opcodes::D2I => RuntimeValue::Int(value as i32 as i64),
                        opcodes::D2L => RuntimeValue::Long(value as i64),
                        _ => RuntimeValue::Float(value as f32),
                    };
                    frame.push_value(converted);
                    pc += 1;
                },
                opcodes::I2B | opcodes::I2C | opcodes::I2D | opcodes::I2F | opcodes::I2L | opcodes::I2S => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => value as i32,
                        _ => return Err("integer conversion requires an integer".to_string()),
                    };
                    let converted = match code[pc] {
                        opcodes::I2B => RuntimeValue::Int(value as i8 as i64),
                        opcodes::I2C => RuntimeValue::Int(value as u16 as i64),
                        opcodes::I2D => RuntimeValue::Double(value as f64),
                        opcodes::I2F => RuntimeValue::Float(value as f32),
                        opcodes::I2L => RuntimeValue::Long(value as i64),
                        _ => RuntimeValue::Int(value as i16 as i64),
                    };
                    frame.push_value(converted);
                    pc += 1;
                },
                opcodes::DADD | opcodes::DSUB | opcodes::DMUL | opcodes::DDIV | opcodes::DREM => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                            let value = match code[pc] {
                                opcodes::DADD => left + right,
                                opcodes::DSUB => left - right,
                                opcodes::DMUL => left * right,
                                opcodes::DDIV => left / right,
                                _ => left % right,
                            };
                            frame.push_value(RuntimeValue::Double(value));
                        },
                        _ => return Err("double arithmetic requires doubles".to_string()),
                    }
                    pc += 1;
                },
                opcodes::DNEG => {
                    match frame.pop_value_force()? {
                        RuntimeValue::Double(value) => frame.push_value(RuntimeValue::Double(-value)),
                        _ => return Err("dneg requires a double".to_string()),
                    }
                    pc += 1;
                },
                opcodes::FCMPL | opcodes::FCMPG => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Float(left), RuntimeValue::Float(right)) => {
                            let result = match left.partial_cmp(&right) {
                                Some(std::cmp::Ordering::Less) => -1,
                                Some(std::cmp::Ordering::Equal) => 0,
                                Some(std::cmp::Ordering::Greater) => 1,
                                None => if code[pc] == opcodes::FCMPL { -1 } else { 1 },
                            };
                            frame.push_value(RuntimeValue::Int(result));
                        },
                        _ => return Err("float comparison requires floats".to_string()),
                    }
                    pc += 1;
                },
                opcodes::DCMPL | opcodes::DCMPG => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    match (left, right) {
                        (RuntimeValue::Double(left), RuntimeValue::Double(right)) => {
                            let result = match left.partial_cmp(&right) {
                                Some(std::cmp::Ordering::Less) => -1,
                                Some(std::cmp::Ordering::Equal) => 0,
                                Some(std::cmp::Ordering::Greater) => 1,
                                None => if code[pc] == opcodes::DCMPL { -1 } else { 1 },
                            };
                            frame.push_value(RuntimeValue::Int(result));
                        },
                        _ => return Err("double comparison requires doubles".to_string()),
                    }
                    pc += 1;
                },
                opcodes::IFNULL | opcodes::IFNONNULL => {
                    let is_null = match frame.pop_value_force()? {
                        RuntimeValue::Null => true,
                        RuntimeValue::Object(_) | RuntimeValue::String(_)
                        | RuntimeValue::ReferenceArray(_) | RuntimeValue::DoubleArray(_)
                        | RuntimeValue::FloatArray(_) | RuntimeValue::LongArray(_) | RuntimeValue::ByteArray(_)
                        | RuntimeValue::ShortArray(_) | RuntimeValue::IntArray(_)
                        | RuntimeValue::CharArray(_) => false,
                        _ => return Err("null branch requires a reference".to_string()),
                    };
                    let taken = if code[pc] == opcodes::IFNULL { is_null } else { !is_null };
                    if taken {
                        let offset = make_int16(code[pc + 1], code[pc + 2]) as i16;
                        pc = (pc as isize + offset as isize) as usize;
                    } else {
                        pc += 3;
                    }
                },
                opcodes::IFEQ | opcodes::IFNE | opcodes::IFLT | opcodes::IFGE | opcodes::IFGT | opcodes::IFLE => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => value,
                        _ => return Err("integer branch requires an integer".to_string()),
                    };
                    let taken = match code[pc] {
                        opcodes::IFEQ => value == 0,
                        opcodes::IFNE => value != 0,
                        opcodes::IFLT => value < 0,
                        opcodes::IFGE => value >= 0,
                        opcodes::IFGT => value > 0,
                        _ => value <= 0,
                    };
                    if taken {
                        let offset = make_int16(code[pc + 1], code[pc + 2]) as i16;
                        pc = (pc as isize + offset as isize) as usize;
                    } else {
                        pc += 3;
                    }
                },
                opcodes::INVOKEDYNAMIC => {
                    let operands = code.get(pc + 1..pc + 5).ok_or("truncated invokedynamic")?;
                    if operands[2] != 0 || operands[3] != 0 {
                        return Err("invalid invokedynamic reserved bytes".to_string());
                    }
                    let index = make_int16(operands[0], operands[1]) as usize;
                    dynamic::invoke_concat(constant_pool, index, frame, jvm)?;
                    pc += 5;
                },
                opcodes::MONITORENTER | opcodes::MONITOREXIT => {
                    let value = frame.pop_value_force()?;
                    execute_monitor(jvm, value, code[pc] == opcodes::MONITORENTER)?;
                    pc += 1;
                },
                opcodes::INSTANCEOF => {
                    let index = make_int16(code[pc + 1], code[pc + 2]) as usize;
                    let target = lookup_class_name(constant_pool, index)?;
                    let value = frame.pop_value_force()?;
                    // Null is assignable for checkcast, but never an instance of a class.
                    let matches = !matches!(value, RuntimeValue::Null)
                        && reference_assignable(jvm, &value, target);
                    frame.push_value(RuntimeValue::Int(if matches { 1 } else { 0 }));
                    pc += 3;
                },
                opcodes::CHECKCAST => {
                    let index = make_int16(code[pc + 1], code[pc + 2]) as usize;
                    let target = lookup_class_name(constant_pool, index)?;
                    let value = frame.stack.last().ok_or("Stack underflow")?;
                    if !reference_assignable(jvm, value, target) {
                        let class = jvm.lookup_class("java/lang/ClassCastException")
                            .ok_or("ClassCastException class not found")?;
                        *jvm.pending_exception.borrow_mut() = Some(RuntimeValue::Object(
                            rc::Rc::new(cell::RefCell::new(class.create_object()))
                        ));
                    }
                    pc += 3;
                },
                opcodes::ATHROW => {
                    let exception = frame.pop_value_force()?;
                    if !matches!(exception, RuntimeValue::Object(_)) {
                        return Err("athrow requires an object".to_string());
                    }
                    *jvm.pending_exception.borrow_mut() = Some(exception);
                },
                opcodes::ICONSTM1 => {
                    frame.push_value(RuntimeValue::Int(-1));
                    pc += 1;
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
                opcodes::SIPUSH => {
                    let operands = code.get(pc + 1..pc + 3).ok_or("truncated sipush")?;
                    let value = make_int16(operands[0], operands[1]) as i16 as i64;
                    frame.push_value(RuntimeValue::Int(value));
                    pc += 3;
                },
                opcodes::PUSHBYTE => {
                    let value = code[pc + 1] as i8 as i64;
                    frame.push_value(RuntimeValue::Int(value));
                    pc += 2;
                },
                opcodes::DRETURN => {
                    return match frame.pop_value_force()? {
                        RuntimeValue::Double(value) => Ok(RuntimeValue::Double(value)),
                        _ => Err("dreturn requires a double".to_string()),
                    };
                },
                opcodes::ARETURN => {
                    let value = frame.pop_value_force()?;
                    match value {
                        RuntimeValue::Object(_) | RuntimeValue::String(_) | RuntimeValue::Null
                        | RuntimeValue::ReferenceArray(_) | RuntimeValue::DoubleArray(_)
                        | RuntimeValue::ByteArray(_) | RuntimeValue::CharArray(_)
                        | RuntimeValue::FloatArray(_) | RuntimeValue::LongArray(_)
                        | RuntimeValue::ShortArray(_) | RuntimeValue::IntArray(_) => return Ok(value),
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
                opcodes::WIDE => {
                    pc = execute_wide(code, pc, frame)?;
                },
                opcodes::SWAP => {
                    swap_values(frame)?;
                    pc += 1;
                },
                opcodes::NOP => {
                    pc += 1;
                },
                opcodes::POP | opcodes::POP2 => {
                    let slots = if code[pc] == opcodes::POP { 1 } else { 2 };
                    pop_slots(frame, slots)?;
                    pc += 1;
                },
                opcodes::DUP2X1 | opcodes::DUP2X2 => {
                    let depth = if code[pc] == opcodes::DUP2X1 { 1 } else { 2 };
                    duplicate_two_slots(frame, depth)?;
                    pc += 1;
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
                opcodes::ASTORE3 => {
                    let value = frame.pop_value_force()?;
                    *frame.locals.get_mut(3).ok_or("invalid astore_3 local index")? = value;
                    pc += 1;
                },
                opcodes::ASTORE2 => {
                    let value = frame.pop_value_force()?;
                    frame.locals[2] = value;
                    pc += 1;
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
                opcodes::ALOAD3 => {
                    let value = frame.locals.get(3).ok_or("invalid aload_3 local index")?.clone();
                    frame.push_value(value);
                    pc += 1;
                },
                opcodes::ALOAD2 => {
                    frame.push_value(frame.locals[2].clone());
                    pc += 1;
                },
                opcodes::ALOAD1 => {
                    pc += 1;
                    let value = frame.locals[1].clone();
                    frame.push_value(value);
                },
                opcodes::LOOKUPSWITCH => {
                    let key = match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => value as i32,
                        _ => return Err("lookupswitch requires an integer key".to_string()),
                    };
                    pc = lookup_switch_target(code, pc, key)?;
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
                opcodes::IFACMPEQ | opcodes::IFACMPNE => {
                    let right = frame.pop_value_force()?;
                    let left = frame.pop_value_force()?;
                    let equal = references_equal(&left, &right)?;
                    let taken = if code[pc] == opcodes::IFACMPEQ { equal } else { !equal };
                    if taken {
                        let offset = make_int16(code[pc + 1], code[pc + 2]) as i16;
                        pc = (pc as isize + offset as isize) as usize;
                    } else {
                        pc += 3;
                    }
                },
                opcodes::IFICOMPAREEQUAL | opcodes::IFICOMPARENOTEQUAL
                | opcodes::IFICOMPAREGREATER | opcodes::IFICOMPARELESSEQUAL => {
                    let compare: fn(i64, i64) -> bool = match code[pc] {
                        opcodes::IFICOMPAREEQUAL => |left, right| left == right,
                        opcodes::IFICOMPARENOTEQUAL => |left, right| left != right,
                        opcodes::IFICOMPAREGREATER => |left, right| left > right,
                        _ => |left, right| left <= right,
                    };
                    pc = do_icompare(frame, pc, make_int16(code[pc + 1], code[pc + 2]) as i16, compare)?;
                },
                opcodes::IFICOMPARELESS => {
                    pc = do_icompare(frame, pc, make_int16(code[pc+1], code[pc+2]) as i16, |i1, i2| i1 < i2)?;
                },
                opcodes::IFICOMPAREGREATEREQUAL => {
                    pc = do_icompare(frame, pc, make_int16(code[pc+1], code[pc+2]) as i16, |i1, i2| i1 >= i2)?;
                },
                opcodes::JSR => {
                    let operands = code.get(pc + 1..pc + 3).ok_or("truncated jsr")?;
                    let offset = make_int16(operands[0], operands[1]) as i16;
                    let target = pc as isize + offset as isize;
                    if target < 0 || target as usize >= code.len() {
                        return Err("jsr target out of bounds".to_string());
                    }
                    frame.push_value(RuntimeValue::ReturnAddress(pc + 3));
                    pc = target as usize;
                },
                opcodes::RET => {
                    let index = *code.get(pc + 1).ok_or("truncated ret")? as usize;
                    pc = match frame.locals.get(index) {
                        Some(RuntimeValue::ReturnAddress(address)) if *address < code.len() => *address,
                        Some(RuntimeValue::ReturnAddress(_)) => return Err("ret target out of bounds".to_string()),
                        _ => return Err("ret requires a returnAddress local".to_string()),
                    };
                },
                opcodes::GOTO => {
                    let offset = make_int16(code[pc+1], code[pc+2]) as i16;
                    pc = (pc as isize + offset as isize) as usize;
                },
                opcodes::GOTOW => {
                    let offset = make_int32(code[pc+1], code[pc+2], code[pc+3], code[pc+4]) as i32;
                    pc = (pc as isize + offset as isize) as usize;
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
                opcodes::IOR | opcodes::IAND | opcodes::IXOR
                | opcodes::ISHL | opcodes::ISHR | opcodes::IUSHR => {
                    // Runtime ints use i64 storage, but these operations act on 32 bits.
                    let operation: fn(i64, i64) -> i64 = match code[pc] {
                        opcodes::IOR => |left, right| ((left as i32) | (right as i32)) as i64,
                        opcodes::IAND => |left, right| ((left as i32) & (right as i32)) as i64,
                        opcodes::IXOR => |left, right| ((left as i32) ^ (right as i32)) as i64,
                        opcodes::ISHL => |left, right| ((left as i32) << (right as u32 & 0x1f)) as i64,
                        opcodes::ISHR => |left, right| ((left as i32) >> (right as u32 & 0x1f)) as i64,
                        // Reinterpret the zero-filled result as a signed Java int.
                        _ => |left, right| ((left as u32) >> (right as u32 & 0x1f)) as i32 as i64,
                    };
                    let value = do_iop(frame, operation)?;
                    frame.push_value(value);
                    pc += 1;
                },
                opcodes::INEG => {
                    let value = match frame.pop_value_force()? {
                        RuntimeValue::Int(value) => (value as i32).wrapping_neg() as i64,
                        _ => return Err("ineg requires an integer".to_string()),
                    };
                    frame.push_value(RuntimeValue::Int(value));
                    pc += 1;
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
                opcodes::PUTSTATIC => {
                    let operands = code.get(pc + 1..pc + 3).ok_or("truncated putstatic")?;
                    let index = make_int16(operands[0], operands[1]) as usize;
                    op_putstatic(constant_pool, frame, jvm, index)?;
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
                    push_runtime_constant(constant_pool, frame, jvm, index)?;
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
                RuntimeValue::Int(i) | RuntimeValue::Long(i) => {
                    println!("{}", i);
                },
                RuntimeValue::Float(value) => {
                    if value.is_infinite() {
                        println!("{}Infinity", if value.is_sign_negative() { "-" } else { "" });
                    } else {
                        println!("{:?}", value);
                    }
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
        source: None,
        class: "java/io/PrintStream".to_string(),
        super_class: Some("java/lang/Object".to_string()),
        methods: methods,
        fields: cell::RefCell::new(fields),
    }
}

fn create_java_lang_system<'a>() -> JVMClass<'a> {
    let mut fields = HashMap::new();

    fields.insert("out".to_string(), RuntimeValue::Object(create_stdout_object()));

    let mut methods = HashMap::new();

    return JVMClass{
        source: None,
        class: "java/lang/System".to_string(),
        super_class: Some("java/lang/Object".to_string()),
        methods: methods,
        fields: cell::RefCell::new(fields)
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
        source: None,
        class: "java/lang/Object".to_string(),
        super_class: None,
        methods: methods,
        fields: cell::RefCell::new(fields),
    };
}

fn create_runtime_const<'a>() -> RuntimeConst<'a> {
    let mut classes = HashMap::new();

    classes.insert("java/lang/System".to_string(), create_java_lang_system());
    classes.insert("java/io/PrintStream".to_string(), create_java_io_print_stream());
    classes.insert("java/lang/Object".to_string(), create_java_lang_object());

    for (name, parent) in [("java/lang/Throwable", "java/lang/Object"),
                           ("java/lang/Exception", "java/lang/Throwable"),
                           ("java/lang/RuntimeException", "java/lang/Exception"),
                           ("java/lang/ClassCastException", "java/lang/RuntimeException"),
                           ("java/lang/ArithmeticException", "java/lang/RuntimeException"),
                           ("java/lang/NullPointerException", "java/lang/RuntimeException"),
                           ("java/lang/IllegalMonitorStateException", "java/lang/RuntimeException")] {
        let mut class = create_java_lang_object();
        class.class = name.to_string();
        class.super_class = Some(parent.to_string());
        classes.insert(name.to_string(), class);
    }

    return RuntimeConst{
        classes: classes,
        pending_exception: cell::RefCell::new(None),
        interned_strings: cell::RefCell::new(HashMap::new()),
        monitors: cell::RefCell::new(Vec::new()),
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

