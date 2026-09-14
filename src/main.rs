use std::env;

mod jvm;

use jvm::exec::*;

mod debug;

fn main() {
    let mut class_file = None;
    let mut options = true;
    for arg in env::args().skip(1) {
        if options && arg == "-v" {
            debug::set_verbose(true);
        } else if options && arg == "--" {
            options = false;
        } else if (options && arg.starts_with('-')) || class_file.is_some() {
            eprintln!("Usage: jvm [-v] <class-file>");
            std::process::exit(2);
        } else {
            class_file = Some(arg);
        }
    }

    if let Some(class_file) = class_file {
        if let Err(err) = execute_class_file(&class_file, "main") {
            println!("Error: {0}", err);
        }
    }
}
