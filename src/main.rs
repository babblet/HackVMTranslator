use std::string::String;
use hack_vm_translator::std::Arguments;
use hack_vm_translator::std::Parser;
use hack_vm_translator::helpers;

fn main() {
    let arguments: Arguments = match Arguments() {
        Ok(args) => args,
        Err(e) => panic!("Error when parsing arguments: {}", e);
    }
    println!("Hello, world!");
}
