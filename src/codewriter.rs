use super::commandtype::CommandType;
use std::fs::File;
use std::path::Path;
use std::ffi::OsString;
pub struct CodeWriter {
    out_file: File,
}

impl CodeWriter {
    pub fn new(path: &Path) -> CodeWriter {
        let file: File = match File::create(path) {
            Ok(file) => file,
            Err(e) => panic!("Error when creating file: {}", e),
        };
        return CodeWriter {
            out_file: file,
        }
    }

    pub fn write_arithmetic(&self, command: &OsString) {
        let mut buffer: OsString = OsString::new();
        if command == "add" {
            buffer.push(format!("@SP\n"));
            buffer.push(format!("A=M\n"));
            buffer.push(format!("D=M\n"));
            buffer.push(format!("A=A-1\n"));
            buffer.push(format!("D=M-D\n"));
            buffer.push(format!("M=M+1\n"));
            buffer.push(format!("A=M\n"));
        }
    }

    pub fn write_push_pop(&self, command: CommandType, segment: &OsString, index: &u16) {
        if command == CommandType::PUSH {
            let mut buffer: OsString = OsString::new();
            if segment == "constant" {
                buffer.push(format!("@{}\n", index));
                buffer.push(format!("D=A\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=D\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M+1\n"));
            }
        } else if command == CommandType::POP {

        }
    }

    pub fn close() {

    }
}