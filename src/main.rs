
use std::ffi::OsString;
use hack_vm_translator::std::Arguments;

fn main() {
    let environment_arguments: Vec<OsString> = std::env::args_os().collect();
    let arguments: Arguments = match Arguments::new(environment_arguments) {
        Ok(args) => args,
        Err(e) => panic!("Error when parsing arguments: {}", e),
    };
    println!("Hello, world!");
}
