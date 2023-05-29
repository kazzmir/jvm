use std::io;
use std::env;

fn read_u32_le(file: &mut dyn std::io::Read) -> u32 {
    let mut buf = [0; 4];
    file.read_exact(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

fn dump_jimage(filename: &str) -> Result<(), io::Error>{
    let mut file = std::fs::File::open(filename)?;

    let magic = read_u32_le(&mut file);
    println!("magic: 0x{:08x}", magic);

    if magic != 0xcafedada {
        return Err(io::Error::new(io::ErrorKind::Other, "Not a jimage file"));
    }

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
