use std::env;
use std::io::Read;

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
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Vec<AttributeInfo>,
}

struct AttributeInfo {
    attribute_name_index: u16,
    attribute_length: u32,
    info: Vec<u8>,
}

struct JVMClassFile {
    magic: u32,
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<ConstantPoolInfo>,
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

// jvm class specification
// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-4.html
fn parse_class_file(filename: &str) {
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
                let mut constant_pool_info = ConstantPoolInfo {
                    tag: 0,
                    info: Vec::new(),
                };
                constant_pool_info.tag = read_u8(&mut file);
                println!("Read constant tag {0}", constant_pool_info.tag);
                match constant_pool_info.tag {
                    CONSTANT_Classref => {
                        // name index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                    },
                    CONSTANT_Methodref => {
                        // class index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                        // name and type index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                    },
                    CONSTANT_Utf8 => {
                        // length
                        // constant_pool_info.info.push(read_u8(&mut file));
                        // constant_pool_info.info.push(read_u8(&mut file));
                        let length = read_u16_bigendian(&mut file);
                        // bytes
                        // let length = (constant_pool_info.info[0] as u16) << 8 | constant_pool_info.info[1] as u16;
                        for _i in 0..length {
                            constant_pool_info.info.push(read_u8(&mut file));
                        }
                    },
                    CONSTANT_String => {
                        // string index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                    },
                    CONSTANT_Fieldref => {
                        // class index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                        // name and type index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                    },
                    CONSTANT_NameAndType => {
                        // name index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                        // descriptor index
                        constant_pool_info.info.push(read_u8(&mut file));
                        constant_pool_info.info.push(read_u8(&mut file));
                    },
                    _ => {
                        println!("ERROR: Unhandled constant tag {0}", constant_pool_info.tag);
                    }
                }
                // constant_pool_info.info = read_u8(&mut file);
                jvm_class_file.constant_pool.push(constant_pool_info);
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
            jvm_class_file.methods_count = read_u16_bigendian(&mut file);
            jvm_class_file.attributes_count = read_u16_bigendian(&mut file);

            println!("Magic: 0x{0:x}", jvm_class_file.magic);
            println!("Version: {0}.{1}", jvm_class_file.major_version, jvm_class_file.minor_version);
            println!("Constant pool: {0}", jvm_class_file.constant_pool_count);
            println!("Access flags: 0x{0:X}", jvm_class_file.access_flags);
            println!("Interfaces: {0}", jvm_class_file.interfaces_count);
            println!("Fields: {0}", jvm_class_file.fields_count);
            println!("Methods: {0}", jvm_class_file.methods_count);
        },
        Err(error) => println!("Error opening file: {error}")
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
