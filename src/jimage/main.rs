use std::io;
use std::env;

use myjvm::jvm;

struct JImageHeader {
    magic: u32,
    version: u32,
    flags: u32,
    resource_count: u32,
    table_length: u32,
    location_size: u32,
    strings_size: u32,
}

const HEADER_SLOTS:u32 = 7;

impl JImageHeader {
    fn get_header_size(self: &JImageHeader) -> u32 {
        return HEADER_SLOTS * 4
    }

    fn get_redirect_size(self: &JImageHeader) -> u32 {
        return self.table_length * 4
    }

    fn get_offsets_size(self: &JImageHeader) -> u32 {
        return self.table_length * 4
    }

    fn get_locations_size(self: &JImageHeader) -> u32 {
        return self.location_size
    }

    fn get_strings_size(self: &JImageHeader) -> u32 {
        return self.strings_size
    }

    fn get_redirect_offset(self: &JImageHeader) -> u32 {
        return self.get_header_size()
    }

    fn get_offsets_offset(self: &JImageHeader) -> u32 {
        return self.get_redirect_offset() + self.get_redirect_size()
    }

    fn get_locations_offset(self: &JImageHeader) -> u32 {
        return self.get_offsets_offset() + self.get_offsets_size()
    }

    fn get_strings_offset(self: &JImageHeader) -> u32 {
        return self.get_locations_offset() + self.get_locations_size()
    }

    fn get_index_size(self: &JImageHeader) -> u32 {
        return self.get_header_size() + self.get_redirect_size() + self.get_offsets_size() + self.get_locations_size() + self.get_strings_size();
    }
}

fn read_u32_le(file: &mut dyn std::io::Read) -> u32 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn read_jimage_header(file:&mut std::fs::File) -> Result<JImageHeader, io::Error> {
    let magic = read_u32_le(file);
    println!("magic: 0x{:08x}", magic);

    if magic != 0xcafedada {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a jimage file"));
    }

    let version = read_u32_le(file);

    let major_version = version >> 16;
    let minor_version = version & 0xffff;

    let flags = read_u32_le(file);

    let resource_count = read_u32_le(file);
    let table_length = read_u32_le(file);
    let location_size = read_u32_le(file);
    let strings_size = read_u32_le(file);

    println!("version: {}.{}", major_version, minor_version);
    println!("flags: 0x{:08x}", flags);
    println!("resource_count: {}", resource_count);
    println!("table_length: {}", table_length);
    println!("location_size: {}", location_size);
    println!("strings_size: {}", strings_size);

    Ok(JImageHeader{
        magic: magic,
        version: version,
        flags: flags,
        resource_count: resource_count,
        table_length: table_length,
        location_size: location_size,
        strings_size: strings_size,
    })
}

fn dump_jimage(filename: &str) -> Result<(), io::Error>{
    let mut file = std::fs::File::open(filename)?;

    let header = read_jimage_header(&mut file)?;

    return Ok(());
}

fn main(){
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..] {
        match dump_jimage(arg) {
            Err(e) => {
                println!("Error processing '{}': {}", arg, e);
            }
            Ok(_) => {}
        }
    }
}
