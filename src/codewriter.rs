use super::commandtype::CommandType;
use std::fs::File;
use std::path::Path;
use std::ffi::OsString;
pub struct CodeWriter {
    out_file: File,
}

impl CodeWriter {
    pub fn new(path: &Path) -> CodeWriter {
        return CodeWriter {
            out_file: File::open(path).unwrap(),
        }
    }

    pub fn write_arithmetic(&self, command: &OsString) {
        println!("Write Arithmetic");
    }

    pub fn write_push_pop(&self, command: CommandType, segment: &OsString, index: &u16) {

    }

    pub fn close() {

    }
}