use std::env;
// use std::io::{Read, BufReader};
use std::io::{Read};

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
    attributes: Vec<AttributeInfo>,
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

enum ConstantPool {
    Classref(u16),
    Methodref(u16, u16),
    NameAndType(u16, u16),
    Utf8(String),
    Fieldref(u16, u16),
    Stringref(u16),
}

struct JVMClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<ConstantPool>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Vec<u16>,
    fields_count: u16,
    fields: Vec<FieldInfo>,
    methods_count: u16,
    methods: Vec<MethodInfo>,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

fn read_u32_bigendian(file: &mut std::fs::File) -> u32 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    u32::from_be_bytes(buf)
}

fn read_u16_bigendian(file: &mut std::fs::File) -> u16 {
    let mut buf = [0; 2];
    file.read_exact(&mut buf).unwrap();
    u16::from_be_bytes(buf)
}

fn read_u8(file: &mut std::fs::File) -> u8 {
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

fn read_attribute(file: &mut std::fs::File) -> Result<AttributeInfo, std::io::Error> {

    let name_index = read_u16_bigendian(file);
    let length = read_u32_bigendian(file);

    let result = file.bytes().take(length as usize).map(|r| r.unwrap()).collect::<Vec<_>>();

    return Ok(AttributeInfo{
        attribute_name_index: name_index,
        attribute_length: length,
        info: result,
    });

    // return Err(std::io::Error::new(std::io::ErrorKind::Other, "Not implemented"));
}

fn read_field(file: &mut std::fs::File) -> Result<FieldInfo, std::io::Error> {
    let access_flags = read_u16_bigendian(file);
    let name_index = read_u16_bigendian(file);
    let descriptor_index = read_u16_bigendian(file);
    let attributes_count = read_u16_bigendian(file);

    let mut attributes = Vec::new();
    for _i in 0..attributes_count {
        attributes.push(read_attribute(file)?);
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
    println!("Parsing file: {}", filename);
    // open filename as binary

    match std::fs::File::open(filename) {
        Ok(file) => {
            let mut file = file;
            let mut jvm_class_file = JVMClassFile {
                magic: 0,
                minor_version: 0,
                major_version: 0,
                constant_pool_count: 0,
                constant_pool: Vec::new(),
                access_flags: 0,
                this_class: 0,
                super_class: 0,
                interfaces_count: 0,
                interfaces: Vec::new(),
                fields_count: 0,
                fields: Vec::new(),
                methods_count: 0,
                methods: Vec::new(),
                attributes_count: 0,
                attributes: Vec::new(),
            };
            jvm_class_file.magic = read_u32_bigendian(&mut file);
            jvm_class_file.minor_version = read_u16_bigendian(&mut file);
            jvm_class_file.major_version = read_u16_bigendian(&mut file);
            jvm_class_file.constant_pool_count = read_u16_bigendian(&mut file);

            println!("Reading constants {0}", jvm_class_file.constant_pool_count);
            for _i in 0..jvm_class_file.constant_pool_count - 1 {
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
                        jvm_class_file.constant_pool.push(ConstantPool::Classref(name));
                    },
                    CONSTANT_Methodref => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPool::Methodref(class, name_and_type));
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
                                jvm_class_file.constant_pool.push(ConstantPool::Utf8(s));
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
                        jvm_class_file.constant_pool.push(ConstantPool::Stringref(index));
                    },
                    CONSTANT_Fieldref => {
                        // class index
                        let class = read_u16_bigendian(&mut file);
                        // name and type index
                        let name_and_type = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPool::Fieldref(class, name_and_type));
                    },
                    CONSTANT_NameAndType => {
                        // name index
                        let name = read_u16_bigendian(&mut file);
                        // descriptor index
                        let descriptor = read_u16_bigendian(&mut file);
                        jvm_class_file.constant_pool.push(ConstantPool::NameAndType(name, descriptor));
                    },
                    _ => {
                        println!("ERROR: Unhandled constant tag {0}", tag);
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
                jvm_class_file.fields.push(read_field(&mut file)?);
            }

            jvm_class_file.methods_count = read_u16_bigendian(&mut file);

            for _i in 0..jvm_class_file.methods_count {
                let access_flags = read_u16_bigendian(&mut file);
                let name_index = read_u16_bigendian(&mut file);
                let descriptor_index = read_u16_bigendian(&mut file);
                let attributes_count = read_u16_bigendian(&mut file);

                let mut attributes:Vec<AttributeInfo> = Vec::new();

                for _j in 0..attributes_count {
                    attributes.push(read_attribute(&mut file)?);
                }

                let method = MethodInfo {
                    access_flags: access_flags,
                    name_index: name_index,
                    descriptor_index: descriptor_index,
                    attributes: attributes,
                };

                jvm_class_file.methods.push(method);
            }

            jvm_class_file.attributes_count = read_u16_bigendian(&mut file);

            for _i in 0..jvm_class_file.attributes_count {
                println!("Reading class attribute");
                jvm_class_file.attributes.push(read_attribute(&mut file)?);
            }

            println!("Magic: 0x{0:x}", jvm_class_file.magic);
            println!("Version: {0}.{1}", jvm_class_file.major_version, jvm_class_file.minor_version);
            println!("Constant pool: {0}", jvm_class_file.constant_pool_count);
            println!("Access flags: 0x{0:X}", jvm_class_file.access_flags);
            println!("Interfaces: {0}", jvm_class_file.interfaces_count);
            println!("Fields: {0}", jvm_class_file.fields_count);
            println!("Methods: {0}", jvm_class_file.methods_count);

            return Ok(jvm_class_file)
        },
        Err(error) => {
            println!("Error opening file: {error}");
            return Err(error);
        }
    }
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
        parse_class_file(args[1].as_str());
    }
}
