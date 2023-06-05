use std::io::{Read};

use crate::debug;

pub struct ConstantPoolInfo {
    tag: u8,
    info: Vec<u8>,
}

pub struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeKind>,
}

pub struct MethodInfo {
    access_flags: u16,
    pub name_index: u16,
    descriptor_index: u16,
    pub attributes: Vec<AttributeKind>,
}

pub enum Descriptor {
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

pub struct MethodDescriptor {
    pub parameters: Vec<Descriptor>,
    pub return_type: Descriptor,
}

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

/*
struct AttributeInfo {
    attribute_name_index: u16,
    info: Vec<u8>,
}
*/

pub enum ConstantPoolEntry {
    Classref(u16),
    Methodref(u16, u16),
    NameAndType{name_index:u16, descriptor_index:u16},
    Utf8(String),
    Fieldref{class_index:u16, name_and_type_index:u16},
    Stringref(u16),
}

impl ConstantPoolEntry {
    pub fn name(&self) -> &str {
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

pub type ConstantPool = Vec<ConstantPoolEntry>;

pub struct JVMClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    pub constant_pool: ConstantPool,
    access_flags: u16,
    pub this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<AttributeKind>,
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

const CONSTANT_CLASSREF:u8 = 7;
const CONSTANT_METHODREF:u8 = 10;
const CONSTANT_NAMEANDTYPE:u8 = 12;
const CONSTANT_UTF8:u8 = 1;
const CONSTANT_FIELDREF:u8 = 9;
const CONSTANT_STRING:u8 = 8;

pub struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

// FIXME: change this to an enum
pub struct StackMapFrameEntry{
    // TODO
}

pub enum AttributeKind {
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
            // result.bytes().collect::<Vec<_>>();
            result.bytes().for_each(drop);
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
            // result.bytes().collect::<Vec<_>>();
            result.bytes().for_each(drop);
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
pub fn parse_class_file(filename: &str) -> Result<JVMClassFile, std::io::Error> {
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
                interfaces: Vec::new(),
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
                    CONSTANT_CLASSREF => {
                        // name index
                        let name = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Classref(name));
                    },
                    CONSTANT_METHODREF => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Methodref(class, name_and_type));
                    },

                    CONSTANT_UTF8 => {
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
                    CONSTANT_STRING => {
                        // string index
                        let index = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Stringref(index));
                    },
                    CONSTANT_FIELDREF => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        // name and type index
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPoolEntry::Fieldref{class_index:class, name_and_type_index:name_and_type});
                    },
                    CONSTANT_NAMEANDTYPE => {
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
            let interfaces_count = read_u16_bigendian(&mut file);

            // read interfaces_count number of u16 and put them the interfaces vec
            for _i in 0..interfaces_count {
                jvm_class_file.interfaces.push(read_u16_bigendian(&mut file));
            }

            let fields_count = read_u16_bigendian(&mut file);

            for _i in 0..fields_count {
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
            debug!("Interfaces: {0}", interfaces_count);
            debug!("Fields: {0}", fields_count);
            debug!("Methods: {0}", methods_count);

            for i in 0..fields_count {
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

pub fn lookup_utf8_constant(constant_pool: &ConstantPool, constant_index: usize) -> Option<&str> {
    match constant_pool_lookup(constant_pool, constant_index) {
        Some(ConstantPoolEntry::Utf8(name)) => {
            return Some(name);
        }
        _ => {
            return None;
        }
    }
}

pub fn constant_pool_lookup(constant_pool: &ConstantPool, constant_index: usize) -> Option<&ConstantPoolEntry> {
    if constant_index >= 1 && constant_index <= constant_pool.len() {
        return Some(&constant_pool[constant_index-1]);
    }

    return None
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

pub fn parse_method_descriptor(descriptor: &str) -> Result<MethodDescriptor, String> {
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

pub fn lookup_method_name(constant_pool: &ConstantPool, index: usize) -> Result<String, String> {
    if let Some(method_name) = lookup_utf8_constant(constant_pool, index) {
        return Ok(method_name.to_string());
    }

    return Err(format!("no such method name at index {}", index));
}

