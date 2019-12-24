use super::commandtype::CommandType;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;

pub struct CodeWriter {
    out_file: File,
}

impl CodeWriter {
    pub fn new(path: &Path) -> CodeWriter {
        let mut file: File = match File::create(path) {
            Ok(file) => file,
            Err(e) => panic!("Error when creating file: {}", e),
        };

        //Init Pointers
        let mut buffer: OsString = OsString::new();

        //Stack Segment Pointer
        buffer.push("@256\n");
        buffer.push("D=A\n");
        buffer.push("@0\n");
        buffer.push("M=D\n");

        //Local Segment Pointer
        buffer.push("@2048\n");
        buffer.push("D=A\n");
        buffer.push("@1\n");
        buffer.push("M=D\n");

        if let Some(buffer) = buffer.to_str() {
            match file.write(buffer.as_bytes()) {
                Err(e) => panic!("Error: {}", e),
                _ => ()
            }
        }


        return CodeWriter {
            out_file: file,
        }
    }

    pub fn write(&mut self, command: CommandType, segment: &OsString, index: &u16) {
        let mut buffer: OsString = OsString::new();
        match command {
            CommandType::ADD => {
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("A=A-1\n"));
                buffer.push(format!("D=M+D\n"));
                buffer.push(format!("M=D\n"));
                buffer.push(format!("D=A\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=D+1\n"));
            },
            CommandType::NEG => {

            },
            CommandType::EQ => {

            },
            CommandType::GT => {

            },
            CommandType::LT => {

            },
            CommandType::OR => {

            },
            CommandType::AND => {

            },
            CommandType::NOT => {

            },
            CommandType::PUSH => {
                if segment == "constant" {
                    buffer.push(format!("@{}\n", index));
                    buffer.push(format!("D=A\n"));
                    buffer.push(format!("@SP\n"));
                    buffer.push(format!("A=M\n"));
                    buffer.push(format!("M=D\n"));
                    buffer.push(format!("@SP\n"));
                    buffer.push(format!("M=M+1\n"));
                }
            },
            CommandType::POP => {

            },
            _ => panic!("Error: CodeWriter::write_arithmetic() failed switch case"),
        }

        if let Some(buffer) = buffer.to_str() {
            match self.out_file.write(buffer.as_bytes()) {
                Err(e) => panic!("Error: {}", e),
                _ => ()
            }
        }
    }

    pub fn close() {

    }
}