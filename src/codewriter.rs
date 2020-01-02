use super::commandtype::CommandType;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;

pub struct CodeWriter {
    cc: u16,
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

        //Setup Stack Segment Pointer
        buffer.push("@256\n");
        buffer.push("D=A\n");
        buffer.push("@SP\n");
        buffer.push("M=D\n");

        //Setup Local Segment Pointer
        buffer.push("@2048\n");
        buffer.push("D=A\n");
        buffer.push("@LCL\n");
        buffer.push("M=D\n");

        //Setup Argument Segment Pointer
        buffer.push("@4096\n");
        buffer.push("D=A\n");
        buffer.push("@ARG\n");
        buffer.push("M=D\n");

        if let Some(buffer) = buffer.to_str() {
            match file.write(buffer.as_bytes()) {
                Err(e) => panic!("Error: {}", e),
                _ => ()
            }
        }

        return CodeWriter {
            cc: 0,
            out_file: file,
        }
    }

    pub fn write(&mut self, command: CommandType, segment: &OsString, index: &i16) {
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
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("A=A-1\n"));
                buffer.push(format!("D=M-D\n"));
                buffer.push(format!("M=D\n"));
                buffer.push(format!("D=A\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=D+1\n"));
            },
            CommandType::EQ => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("A=A-1\n"));
                buffer.push(format!("D=M-D\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));
                buffer.push(format!("@COND{}\n", self.cc));
                buffer.push(format!("D;JEQ\n"));

                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));

                buffer.push(format!("(COND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=-1\n"));
                buffer.push(format!("(CONDEND{})\n", self.cc));
            },
            CommandType::GT => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("A=A-1\n"));
                buffer.push(format!("D=M-D\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));
                buffer.push(format!("@COND{}\n", self.cc));
                buffer.push(format!("D;JGT\n"));

                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));

                buffer.push(format!("(COND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=-1\n"));
                buffer.push(format!("(CONDEND{})\n", self.cc));
            },
            CommandType::LT => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("A=A-1\n"));
                buffer.push(format!("D=M-D\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));
                buffer.push(format!("@COND{}\n", self.cc));
                buffer.push(format!("D;JLT\n"));

                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));

                buffer.push(format!("(COND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M-1\n"));
                buffer.push(format!("M=-1\n"));
                buffer.push(format!("(CONDEND{})\n", self.cc));
            },
            CommandType::OR => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));
                buffer.push(format!("A=M\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@OR1{}\n", self.cc));
                buffer.push(format!("D+1;JEQ\n"));

                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));
                buffer.push(format!("A=M\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@OR2{}\n", self.cc));
                buffer.push(format!("D+1;JEQ\n"));

                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));


                buffer.push(format!("(OR1{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M-1\n"));

                buffer.push(format!("(OR2{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=-1\n"));
                buffer.push(format!("(CONDEND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M+1\n"));
            },
            CommandType::AND => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n")); //258
                buffer.push(format!("M=M-1\n")); //257
                buffer.push(format!("A=M\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("M=0\n")); //257 = 0
                buffer.push(format!("@COND1{}\n", self.cc));
                buffer.push(format!("D;JEQ\n")); //false

                buffer.push(format!("@SP\n")); //257
                buffer.push(format!("M=M-1\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("@COND2{}\n", self.cc));
                buffer.push(format!("D;JEQ\n")); // false

                buffer.push(format!("@SP\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=-1\n")); // true
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));
		
                buffer.push(format!("(COND1{})\n", self.cc)); // false
                buffer.push(format!("@SP\n")); //257
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@SP\n")); //257
                buffer.push(format!("M=M-1\n")); //256
		
                buffer.push(format!("(COND2{})\n", self.cc)); // false
                buffer.push(format!("@SP\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=0\n"));

		        buffer.push(format!("(CONDEND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M+1\n"));
            },
            CommandType::NOT => {
                self.cc = self.cc + 1;
                buffer.push(format!("@SP\n")); //257
                buffer.push(format!("M=M-1\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("D=M\n"));
                buffer.push(format!("@CONDTRUE{}\n", self.cc));
                buffer.push(format!("D;JEQ\n"));

                buffer.push(format!("\n"));
                buffer.push(format!("@SP\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=0\n"));
                buffer.push(format!("@CONDEND{}\n", self.cc));
                buffer.push(format!("0;JEQ\n"));

                buffer.push(format!("(CONDTRUE{})\n", self.cc));
                buffer.push(format!("@SP\n")); //256
                buffer.push(format!("A=M\n"));
                buffer.push(format!("M=-1\n"));

		        buffer.push(format!("(CONDEND{})\n", self.cc));
                buffer.push(format!("@SP\n"));
                buffer.push(format!("M=M+1\n"));
            },
            CommandType::PUSH => {
                if segment == "constant" {
                    if index.clone() == -1 {
                        buffer.push(format!("D=-1\n"));
                    } else {
                        buffer.push(format!("@{}\n", index));
                        buffer.push(format!("D=A\n"));
                    }
                    
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
}
