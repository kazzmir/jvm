use std::env;
// use std::io::{Read, BufReader};
use std::io::{Read};
use std::collections::HashMap;
use std::fmt;
use std::rc;
use std::cell;
use debug_print::debug_eprintln as debug;

/*
ClassFile {
    u4             magic;
    u2             minor_version;
    u2             major_version;
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    u2             super_class;
    u2             interfaces_count;
    u2             interfaces[interfaces_count];
    u2             fields_count;
    field_info     fields[fields_count];
    u2             methods_count;
    method_info    methods[methods_count];
    u2             attributes_count;
    attribute_info attributes[attributes_count];
}
*/

struct ConstantPoolInfo {
    tag: u8,
    info: Vec<u8>,
}

struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeKind>,
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeKind>,
}

enum Descriptor {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Object(String),
    Void,
    Array(Box<Descriptor>),
}

struct MethodDescriptor {
    parameters: Vec<Descriptor>,
    return_type: Descriptor,
}

/*
struct AttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}
*/

enum ConstantPoolEntry {
    Classref(u16),
    Methodref(u16, u16),
    NameAndType{name_index:u16, descriptor_index:u16},
    Utf8(String),
    Fieldref{class_index:u16, name_and_type_index:u16},
    Stringref(u16),
}

impl ConstantPoolEntry {
    fn name(&self) -> &str {
        return match self {
            ConstantPoolEntry::Classref(_) => "Classref",
            ConstantPoolEntry::Methodref(_, _) => "Methodref",
            ConstantPoolEntry::NameAndType{..} => "NameAndType",
            ConstantPoolEntry::Utf8(_) => "Utf8",
            ConstantPoolEntry::Fieldref{..} => "Fieldref",
            ConstantPoolEntry::Stringref(_) => "Stringref",
        }
    }
}

type ConstantPool = Vec<ConstantPoolEntry>;

struct JVMClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool: ConstantPool,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeKind>,
}

fn read_u32_bigendian(file: &mut dyn std::io::Read) -> u32 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    u32::from_be_bytes(buf)
}

fn read_u16_bigendian(file: &mut dyn std::io::Read) -> u16 {
    let mut buf = [0; 2];
    file.read_exact(&mut buf).unwrap();
    u16::from_be_bytes(buf)
}

fn read_u8(file: &mut dyn std::io::Read) -> u8 {
    let mut buf = [0; 1];
    file.read_exact(&mut buf).unwrap();
    u8::from_be_bytes(buf)
}

const CONSTANT_Classref:u8 = 7;
const CONSTANT_Methodref:u8 = 10;
const CONSTANT_NameAndType:u8 = 12;
const CONSTANT_Utf8:u8 = 1;
const CONSTANT_Fieldref:u8 = 9;
const CONSTANT_String:u8 = 8;

struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

// FIXME: change this to an enum
struct StackMapFrameEntry{
    // TODO
}

enum AttributeKind {
    Code{
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeKind>,
    },
    LineNumberTable{
        line_number_table: Vec<LineNumberTableEntry>,
    },
    SourceFile{
    },
    StackMapFrame{
        entries: Vec<StackMapFrameEntry>,
    },
}

fn read_verification_type_info(file: &mut dyn std::io::Read) -> Result<(), std::io::Error> {
    let kind = read_u8(file);
    match kind {
        0 => {
            // Top_variable_info
            return Ok(())
        },
        1 => {
            // Integer_variable_info
            return Ok(())
        },
        2 => {
            // Float_variable_info
            return Ok(())
        },
        3 => {
            // Double_variable_info
            return Ok(())
        },
        4 => {
            // Long_variable_info
            return Ok(())
        },
        5 => {
            // Null_variable_info
            return Ok(())
        },
        6 => {
            // UninitializedThis_variable_info
            return Ok(())
        },
        7 => {
            // Object_variable_info
            let index = read_u16_bigendian(file);
            return Ok(())
        },
        8 => {
            let index = read_u16_bigendian(file);
            return Ok(())
        },
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unknown verification type {}", kind)))
        }
    }
}

fn read_stackmap_frame(file: &mut dyn std::io::Read) -> Result<StackMapFrameEntry, std::io::Error> {
    let kind = read_u8(file);

    if kind <= 63 {
        return Ok(StackMapFrameEntry{})
    }

    if kind >= 64 && kind <= 127 {
        read_verification_type_info(file)?;
        return Ok(StackMapFrameEntry{})
    }

    if kind >= 128 && kind <= 246 {
        // reserved for future use
        return Ok(StackMapFrameEntry{})
    }

    if kind == 247 {
        let offset_delta = read_u16_bigendian(file);
        read_verification_type_info(file)?;
        return Ok(StackMapFrameEntry{})
    }

    if kind >= 248 && kind <= 250 {
        let offset_delta = read_u16_bigendian(file);
        return Ok(StackMapFrameEntry{})
    }

    if kind == 251 {
        let offset_delta = read_u16_bigendian(file);
        return Ok(StackMapFrameEntry{})
    }

    if kind >= 252 && kind <= 254 {
        let offset_delta = read_u16_bigendian(file);
        for _ in 0..(kind - 251) {
            read_verification_type_info(file)?;
        }
        return Ok(StackMapFrameEntry{})
    }

    if kind == 255 {
        let offset_delta = read_u16_bigendian(file);
        let number_of_locals = read_u16_bigendian(file);
        for _ in 0..number_of_locals {
            read_verification_type_info(file)?;
        }
        let number_of_stack_items = read_u16_bigendian(file);
        for _ in 0..number_of_stack_items {
            read_verification_type_info(file)?;
        }
        return Ok(StackMapFrameEntry{})
    }

    return Ok(StackMapFrameEntry{});
}

fn read_exception(file: &mut dyn std::io::Read) -> Result<ExceptionTableEntry, std::io::Error> {
    let start_pc = read_u16_bigendian(file);
    let end_pc = read_u16_bigendian(file);
    let handler_pc = read_u16_bigendian(file);
    let catch_type = read_u16_bigendian(file);
    return Ok(ExceptionTableEntry{
        start_pc: start_pc,
        end_pc: end_pc,
        handler_pc: handler_pc,
        catch_type: catch_type,
    });
}

fn read_attribute(file: &mut dyn std::io::Read, constant_pool: &ConstantPool) -> Result<AttributeKind, std::io::Error> {

    let name_index = read_u16_bigendian(file);
    let length = read_u32_bigendian(file);

    // let result = file.bytes().take(length as usize).map(|r| r.unwrap()).collect::<Vec<_>>();
    let mut result = file.take(length as u64);

    match lookup_utf8_constant(constant_pool, name_index as usize) {
        Some("Code") => {
            let max_stack = read_u16_bigendian(&mut result);
            let max_locals = read_u16_bigendian(&mut result);
            let code_length = read_u32_bigendian(&mut result);
            let code = result.by_ref().take(code_length as u64).bytes().map(|r| r.unwrap()).collect::<Vec<_>>();
            // println!("Read {} bytes of code", code.len());
            let exception_table_length = read_u16_bigendian(&mut result);
            let mut exceptions:Vec<ExceptionTableEntry> = Vec::new();
            for _i in 0..exception_table_length {
                exceptions.push(read_exception(&mut result)?);
            }

            let attributes_length = read_u16_bigendian(&mut result);
            let attributes = read_attributes(&mut result, constant_pool, attributes_length)?;

            let rest = result.bytes().collect::<Vec<_>>();
            if rest.len() > 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} bytes left over", rest.len())));
            }

            return Ok(AttributeKind::Code{
                max_stack: max_stack,
                max_locals: max_locals,
                code: code,
                exception_table: exceptions,
                attributes: attributes,
            });
        },
        Some("LineNumberTable") => {
            // force reading the bytes
            let mut line_numbers:Vec<LineNumberTableEntry> = Vec::new();
            let length = read_u16_bigendian(&mut result);
            for _i in 0..length {
                line_numbers.push(LineNumberTableEntry{
                    start_pc: read_u16_bigendian(&mut result),
                    line_number: read_u16_bigendian(&mut result),
                });
            }
            let rest = result.bytes().collect::<Vec<_>>();
            if rest.len() > 0 {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{} bytes left over", rest.len())));
            }
            return Ok(AttributeKind::LineNumberTable{
                line_number_table: line_numbers,
            });
        },
        Some("SourceFile") => {
            result.bytes().collect::<Vec<_>>();
            return Ok(AttributeKind::SourceFile{
            });
        },
        Some("StackMapTable") => {
            let entry_count = read_u16_bigendian(&mut result);
            let mut entries = Vec::new();
            for _i in 0..entry_count {
                let frame = read_stackmap_frame(&mut result)?;
                entries.push(frame);
            }
            return Ok(AttributeKind::StackMapFrame{
                entries: entries,
            });
        },
        Some(something) => {
            result.bytes().collect::<Vec<_>>();
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unknown attribute '{}'", something)));
        },
        _ => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unhandled name index {}", name_index)));
        }
    }

    /*
    return Ok(AttributeInfo{
        attribute_name_index: name_index,
        info: result,
    });
    */

    // return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented"));
}

fn read_attributes(reader: &mut dyn std::io::Read, constant_pool: &ConstantPool, count: u16) -> Result<Vec<AttributeKind>, std::io::Error> {
    let mut result = Vec::new();
    for _i in 0..count {
        result.push(read_attribute(reader, constant_pool)?);
    }
    return Ok(result);
}

fn read_field(file: &mut std::fs::File, constant_pool: &ConstantPool) -> Result<FieldInfo, std::io::Error> {
    let access_flags = read_u16_bigendian(file);
    let name_index = read_u16_bigendian(file);
    let descriptor_index = read_u16_bigendian(file);
    let attributes_count = read_u16_bigendian(file);

    debug!("Read field attributes");
    let mut attributes = Vec::new();
    for _i in 0..attributes_count {
        attributes.push(read_attribute(file, constant_pool)?);
    }

    return Ok(FieldInfo{
        access_flags: access_flags,
        name_index: name_index,
        descriptor_index: descriptor_index,
        attributes: attributes,
    });
}

// jvm class specification
// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html
fn parse_class_file(filename: &str) -> Result<JVMClassFile, std::io::Error> {
    debug!("Parsing file: {}", filename);
    // open filename as binary

    match std::fs::File::open(filename) {
        Ok(file) => {
            let mut file = file;
            let mut jvm_class_file = JVMClassFile {
                magic: 0,
                minor_version: 0,
                major_version: 0,
                constant_pool: Vec::new(),
                access_flags: 0,
                this_class: 0,
                super_class: 0,
                interfaces_count: 0,
                interfaces: Vec::new(),
                fields_count: 0,
                fields: Vec::new(),
                methods: Vec::new(),
                attributes: Vec::new(),
            };
            jvm_class_file.magic = read_u32_bigendian(&mut file);

            if jvm_class_file.magic != 0xcafebabe {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not a class file"));
            }

            jvm_class_file.minor_version = read_u16_bigendian(&mut file);
            jvm_class_file.major_version = read_u16_bigendian(&mut file);
            let constant_pool_count = read_u16_bigendian(&mut file);

            debug!("Reading constants {0}", constant_pool_count);
            for _i in 0..constant_pool_count - 1 {
                /*
                let mut constant_pool_info = ConstantPoolInfo {
                    tag: 0,
                    info: Vec::new(),
                };
                */
                let tag = read_u8(&mut file);
                // println!("Read constant tag {0}", tag);
                match tag {
                    CONSTANT_Classref => {
                        // name index
                        let name = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Classref(name));
                    },
                    CONSTANT_Methodref => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Methodref(class, name_and_type));
                    },

                    CONSTANT_Utf8 => {
                        // length
                        // constant_pool_info.info.push(read_u8(&mut file));
                        // constant_pool_info.info.push(read_u8(&mut file));
                        let length = read_u16_bigendian(&mut file);
                        // bytes
                        // let length = (constant_pool_info.info[0] as u16) << 8 | constant_pool_info.info[1] as u16;
                        /*
                        for _i in 0..length {
                            constant_pool_info.info.push(read_u8(&mut file));
                        }
                        */

                        // read length bytes from file
                        let result = String::from_utf8((&file).bytes().take(length as usize).map(|r| r.unwrap()).collect::<Vec<_>>());
                        match result {
                            Ok(s) => {
                                // println!("Read utf8 string '{}'", s);
                                jvm_class_file.constant_pool.push(ConstantPoolEntry::Utf8(s));
                            }
                            Err(err) => return Err(std::io::Error::new(std::io::ErrorKind::Other, err))
                        }

                        /*
                        match
                            Ok(s) => {
                            }
                            Err(std::string::FromUtf8Error) => {
                                println!("ERROR: could not read utf8 string: {}", err);
                                return Err(err)
                            }
                        }
                        */

                        /*
                        let bytes = file.bytes().take(length as usize);
                        let reader: Box<dyn Read> = Box::new(bytes);
                        let bufreader = BufReader::new(reader);
                        let mut buffer = String::new();
                        bufreader.read_to_string(&mut buffer);
                        */

                        /*
                        match bytes.collect::<Vec<_>>() {
                            Ok(data) => {
                                jvm_class_file.constant_pool.push(ConstantPool::Utf8(String::from_utf8(data).unwrap()));
                            },
                            Error => {
                                println!("ERROR: reading utf8 string");
                            }
                        }
                        */
                    },
                    CONSTANT_String => {
                        // string index
                        let index = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Stringref(index));
                    },
                    CONSTANT_Fieldref => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        // name and type index
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Fieldref{class_index:class, name_and_type_index:name_and_type});
                    },
                    CONSTANT_NameAndType => {
                        // name index
                        let name = read_u16_bigendian(&mut file);
                        // descriptor index
                        let descriptor = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::NameAndType{name_index:name, descriptor_index:descriptor});
                    },
                    _ => {
                        return Err(std::io::Error::new(std::io::ErrorKind::Other, "Unhandled constant tag"));
                    }
                }
                // constant_pool_info.info = read_u8(&mut file);
                // jvm_class_file.constant_pool.push(constant_pool_info);
            }

            jvm_class_file.access_flags = read_u16_bigendian(&mut file);
            jvm_class_file.this_class = read_u16_bigendian(&mut file);
            jvm_class_file.super_class = read_u16_bigendian(&mut file);
            jvm_class_file.interfaces_count = read_u16_bigendian(&mut file);

            // read interfaces_count number of u16 and put them the interfaces vec
            for _i in 0..jvm_class_file.interfaces_count {
                jvm_class_file.interfaces.push(read_u16_bigendian(&mut file));
            }

            jvm_class_file.fields_count = read_u16_bigendian(&mut file);

            for _i in 0..jvm_class_file.fields_count {
                jvm_class_file.fields.push(read_field(&mut file, &jvm_class_file.constant_pool)?);
            }

            let methods_count = read_u16_bigendian(&mut file);

            for _i in 0..methods_count {
                let access_flags = read_u16_bigendian(&mut file);
                let name_index = read_u16_bigendian(&mut file);
                let descriptor_index = read_u16_bigendian(&mut file);
                let attributes_count = read_u16_bigendian(&mut file);

                debug!("Method access flags 0x{:x}", access_flags);

                let attributes = read_attributes(&mut file, &jvm_class_file.constant_pool, attributes_count)?;

                let method = MethodInfo {
                    access_flags: access_flags,
                    name_index: name_index,
                    descriptor_index: descriptor_index,
                    attributes: attributes,
                };

                jvm_class_file.methods.push(method);
            }

            let attributes_count = read_u16_bigendian(&mut file);

            /*
            for _i in 0..jvm_class_file.attributes_count {
                println!("Reading class attribute");
                jvm_class_file.attributes.push(read_attribute(&mut file, &jvm_class_file.constant_pool)?);
            }
            */

            debug!("Reading class attributes");
            jvm_class_file.attributes = read_attributes(&mut file, &jvm_class_file.constant_pool, attributes_count)?;

            debug!("Magic: 0x{0:x}", jvm_class_file.magic);
            debug!("Version: {0}.{1}", jvm_class_file.major_version, jvm_class_file.minor_version);
            debug!("Constant pool: {0}", constant_pool_count);
            debug!("Access flags: 0x{0:X}", jvm_class_file.access_flags);
            debug!("Interfaces: {0}", jvm_class_file.interfaces_count);
            debug!("Fields: {0}", jvm_class_file.fields_count);
            debug!("Methods: {0}", methods_count);

            for i in 0..jvm_class_file.fields_count {
                debug!("Field {}", i);
                let name = lookup_utf8_constant(&jvm_class_file.constant_pool, jvm_class_file.fields[i as usize].name_index as usize);
                match name {
                    Some(name) => {
                        debug!("  name={}", name);
                    },
                    None => {
                        debug!("  name=unknown");
                    }
                }
                let descriptor = lookup_utf8_constant(&jvm_class_file.constant_pool, jvm_class_file.fields[i as usize].descriptor_index as usize);
                match descriptor {
                    Some(descriptor) => {
                        debug!("  descriptor={}", descriptor);
                    },
                    None => {
                        debug!("  descriptor=unknown");
                    }
                }
            }

            return Ok(jvm_class_file)
        },
        Err(error) => {
            return Err(error);
        }
    }
}

fn lookup_utf8_constant(constant_pool: &ConstantPool, constant_index: usize) -> Option<&str> {
    match constant_pool_lookup(constant_pool, constant_index) {
        Some(ConstantPoolEntry::Utf8(name)) => {
            return Some(name);
        }
        _ => {
            return None;
        }
    }
}

fn constant_pool_lookup(constant_pool: &ConstantPool, constant_index: usize) -> Option<&ConstantPoolEntry> {
    if constant_index >= 1 && constant_index <= constant_pool.len() {
        return Some(&constant_pool[constant_index-1]);
    }

    return None
}

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

fn make_string_from(descriptor: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut string = String::new();

    while let Some(c) = descriptor.peek() {
        string.push(*c);
        descriptor.next();
    }

    return string;
}

fn parse_field_descriptor(descriptor: &mut std::iter::Peekable<std::str::Chars>) -> Result<Descriptor, String> {

    if let Some('B') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Byte);
    }

    if let Some('C') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Char);
    }

    if let Some('D') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Double);
    }

    if let Some('F') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Float);
    }

    if let Some('I') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Int);
    }

    if let Some('J') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Long);
    }

    if let Some('S') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Short);
    }

    if let Some('Z') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Boolean);
    }

    if let Some('V') = descriptor.peek() {
        descriptor.next();
        return Ok(Descriptor::Void);
    }

    if let Some('L') = descriptor.peek() {
        descriptor.next();

        let mut class_name = String::new();

        while let Some(character) = descriptor.next() {
            if character == ';' {
                return Ok(Descriptor::Object(class_name));
            }

            class_name.push(character);
        }

        return Err("Invalid class name".to_string());
    }

    if let Some('[') = descriptor.peek() {
        descriptor.next();

        match parse_field_descriptor(descriptor)? {
            Descriptor::Void => return Err("cannot have an array of void".to_string()),
            d => return Ok(Descriptor::Array(Box::new(d)))
        }
    }

    return Err(format!("cannot parse field descriptor: {}", make_string_from(descriptor)));
}

fn parse_method_descriptor(descriptor: &str) -> Result<MethodDescriptor, String> {
    // println!("parse method descriptor: {}", descriptor);

    let mut parameters = Vec::new();
    let mut descriptor = descriptor.chars().peekable();

    if descriptor.peek() != Some(&'(') {
        return Err("Invalid method descriptor".to_string());
    }

    descriptor.next();

    while descriptor.peek() != Some(&')') {
        parameters.push(parse_field_descriptor(&mut descriptor)?);
    }

    descriptor.next();

    return Ok(MethodDescriptor{
        parameters: parameters,
        return_type: parse_field_descriptor(&mut descriptor)?,
    });
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

fn make_int(byte1:u8, byte2:u8) -> u16 {
    return ((byte1 as u16) << 8) | (byte2 as u16)
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
                Opcodes::IfICompareGreaterEqual => {
                    pc = do_icompare(frame, pc, make_int(code[pc+1], code[pc+2]) as i16, |i1, i2| i1 >= i2)?;
                },
                Opcodes::Goto => {
                    let old = pc;
                    let offset = make_int(code[pc+1], code[pc+2]);
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
