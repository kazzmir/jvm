use std::io;
use std::env;
use std::io::Seek;

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

const ATTRIBUTE_END:u32 = 0;
const ATTRIBUTE_MODULE:u32 = 1;
const ATTRIBUTE_PARENT:u32 = 2;
const ATTRIBUTE_BASE:u32 = 3;
const ATTRIBUTE_EXTENSION:u32 = 4;
const ATTRIBUTE_OFFSET:u32 = 5;
const ATTRIBUTE_COMPRESSED:u32 = 6;
const ATTRIBUTE_UNCOMPRESSED:u32 = 7;
const ATTRIBUTE_COUNT:u32 = 8;

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

fn read_u32_le(file: &mut dyn std::io::Read) -> Result<u32, io::Error> {
    let mut buf = [0; 4];
    file.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u8(file: &mut dyn std::io::Read) -> Result<u8, io::Error> {
    let mut buf = [0; 1];
    file.read_exact(&mut buf)?;
    Ok(u8::from_be_bytes(buf))
}

fn read_jimage_header(file:&mut std::fs::File) -> Result<JImageHeader, io::Error> {
    let magic = read_u32_le(file)?;
    println!("magic: 0x{:08x}", magic);

    if magic != 0xcafedada {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a jimage file"));
    }

    let version = read_u32_le(file)?;

    let major_version = version >> 16;
    let minor_version = version & 0xffff;

    let flags = read_u32_le(file)?;

    let resource_count = read_u32_le(file)?;
    let table_length = read_u32_le(file)?;
    let location_size = read_u32_le(file)?;
    let strings_size = read_u32_le(file)?;

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

fn read_value(length: u8, locations: &Vec<u8>, offset: u32) -> u64 {
    let mut value:u64 = 0;
    for i in 0..length {
        // value |= (locations[(offset + i) as usize] as u64) << (i * 8);
        value = value << 8;
        value |= (locations[(offset + (i as u32)) as usize] as u64) & 0xff;
    }
    return value;
}

fn decompress_entry(locations: &Vec<u8>, offset: u32) -> Result<Vec<u64>, io::Error> {
    let mut offset = offset;

    let mut attributes = Vec::new();

    while offset < locations.len() as u32 {
        let value = locations[offset as usize] & 0xff;
        offset += 1;
        if value < 0x7 {
            break
        }

        let kind = value >> 3;
        let length = (value & 0x7) + 1;
        let attribute = read_value(length, locations, offset);
        offset += length as u32;
        // println!("Read attribute {} {}", kind, attribute);
        attributes.push(attribute);
    }

    // println!("decompress_entry {}", offset);
    Ok(attributes)
    // return Err(io::Error::new(io::ErrorKind::Other, "Not implemented"));
}

fn compute_string_length(strings: &Vec<u8>, offset: u64) -> u64 {
    let mut length = 0;

    let mut offset = offset;
    while offset < strings.len() as u64 {
        let value = strings[offset as usize];
        offset += 1;
        if value == 0 {
            return length
        }

        if value & 0xc0 != 0x80 {
            length += 1;
        }
    }

    return length
}

fn read_string(strings: &Vec<u8>, offset: u64) -> String {
    let length = compute_string_length(strings, offset);

    let mut string = String::new();

    for i in 0..length {
        // FIXME: handle utf8
        let value = strings[(offset + i) as usize];
        string.push(value as char);
    }

    return string
}

fn read_offsets(header: &JImageHeader, file: &mut std::fs::File) -> Result<Vec<u32>, io::Error> {

    println!("read offsets from offset {} size {}", header.get_offsets_offset(), header.get_offsets_size());

    file.seek(io::SeekFrom::Start(header.get_offsets_offset() as u64))?;

    let mut offsets = Vec::new();
    for _i in 0..header.get_offsets_size() {
        offsets.push(read_u32_le(file)?);
    }

    println!("Read {} offsets", offsets.len());

    file.seek(io::SeekFrom::Start(header.get_locations_offset() as u64))?;
    let mut locations = Vec::new();
    for _i in 0..header.get_locations_size() {
        let location = read_u8(file)?;
        locations.push(location);
    }

    println!("Read {} locations", locations.len());

    let mut strings = Vec::new();
    for _i in 0..header.get_strings_size() {
        let string = read_u8(file)?;
        strings.push(string);
}

    println!("Read {} strings", strings.len());

    for offset in offsets {
        let attributes = decompress_entry(&locations, offset)?;
        if attributes.len() == 0 {
            continue
        }
        println!("Attributes: {:?}", attributes);
        if attributes.len() >= 2 {
            println!("  module name: {}", read_string(&strings, attributes[1]));
        }
        if attributes.len() >= 3 {
            println!("  base name: {}", read_string(&strings, attributes[2]));
        }
        if attributes.len() >= 4 {
            println!("  parent name: {}", read_string(&strings, attributes[3]));
        }
    }

    return Err(io::Error::new(io::ErrorKind::Other, "Not implemented"));
}

fn dump_jimage(filename: &str) -> Result<(), io::Error>{
    let mut file = std::fs::File::open(filename)?;

    let header = read_jimage_header(&mut file)?;

    let offsets = read_offsets(&header, &mut file)?;

    println!("Index size: {}", header.get_index_size());

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
