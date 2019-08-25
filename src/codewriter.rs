use super::commandtype::CommandType;
use std::fs::File;
use std::path::Path;
pub struct CodeWriter {
    out_file: File,
}

impl CodeWriter {
    pub fn new(path: &Path) -> CodeWriter {
        return CodeWriter {
            out_file: File::open(path).unwrap(),
        }
    }

    pub fn write_arithmetic(command: String) {
        println!("Write Arithmetic");
    }

    pub fn write_push_pop(command: CommandType, segment: String, index: u16) {

    }

    pub fn close() {

    }
}