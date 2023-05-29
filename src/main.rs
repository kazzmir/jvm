use std::env;
use std::fmt;
use debug_print::debug_eprintln as debug;

mod jvm;

use jvm::data::*;
use jvm::exec::*;

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
