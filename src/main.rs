use std::env;

mod jvm;

use jvm::exec::*;

mod debug;

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
        if let Err(err) = execute_class_file(&args[1], "main") {
            println!("Error: {0}", err);
        }
    }
}
