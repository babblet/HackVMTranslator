use super::commandtype::CommandType;
//use super::address::Address;
//use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ffi::OsString;

pub struct CodeWriter {
  cc: u16,
  out_file: File,
  context_name: Vec<String>,
  context_index: Vec<u16>,
}

impl CodeWriter {
  pub fn new(path: &Path) -> CodeWriter {
    let file: File = match File::create(path) {
      Ok(file) => file,
      Err(e) => panic!("{}", e),
    };

    return CodeWriter {
      cc: 0,
      out_file: file,
      context_name: Vec::new(),
      context_index: Vec::new(),
    }
  }

  fn push_context(&mut self, name: String) {
    self.context_name.push(name);
    self.context_index.push(0);
  }

  fn add_context(&mut self) {
    let x = self.context_index.pop().unwrap() + 1;
    self.context_index.push(x);
  }

  fn pop_context(&mut self) {
    self.context_name.pop();
    self.context_index.pop();
  }

  pub fn write(&mut self, command: CommandType, segment: &OsString, index: i16) -> Result<(), String>{
    let mut buffer: OsString = OsString::new();
    match command {
      CommandType::ADD => {
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("A=A-1\n"));
        buffer.push(format!("D=M+D\n"));
        buffer.push(format!("M=D\n"));
        buffer.push(format!("D=A\n"));
        buffer.push(format!("@SP\n"));
        buffer.push(format!("M=D+1\n"));
      },
      CommandType::SUB => {
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("A=A-1\n"));
        buffer.push(format!("D=M-D\n"));
        buffer.push(format!("M=D\n"));
        buffer.push(format!("D=A\n"));
        buffer.push(format!("@SP\n"));
        buffer.push(format!("M=D+1\n"));
      },
      CommandType::NEG => {
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("M=-D\n"));
      },
      CommandType::EQ => {
        self.cc = self.cc + 1;
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("A=A-1\n"));
        buffer.push(format!("D=M-D\n"));
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
        buffer.push(format!("A=A-1\n"));
        buffer.push(format!("D=M-D\n"));
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
        buffer.push(format!("A=A-1\n"));
        buffer.push(format!("D=M-D\n"));
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
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("M=D|M\n"));
      },
      CommandType::AND => {
        self.cc = self.cc + 1;
        buffer.push(format!("@SP\n"));
        buffer.push(format!("M=M-1\n"));
        buffer.push(format!("A=M\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("M=D&M\n"));
      },
      CommandType::NOT => {
        self.cc = self.cc + 1;
        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("M=!M\n"));
      },
      CommandType::PUSH => {
        if segment.to_str() == Some("constant") {
          if index.clone() == -1 {
            buffer.push(format!("D=-1\n"));
          } else {
            buffer.push(format!("@{}\n", index));
            buffer.push(format!("D=A\n"));
          }
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        } else if segment.to_str() == Some("temp") {
          buffer.push(format!("@{}\n", index + 5));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        } else if segment.to_str() == Some("pointer") {
          match index {
            0 => {
              buffer.push(format!("@THIS\n"));
            },
            1 => {
              buffer.push(format!("@THAT\n"));
            },
            _ => (),
          }
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        } else if segment.to_str() == Some("static") {
          buffer.push(format!("@STATIC{}\n", index));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        } else {
          buffer.push(format!("@{}\n", index));
          buffer.push(format!("D=A\n"));
          match segment.to_str() {
            Some("local") => {
              buffer.push(format!("@LCL\n"));
            },
            Some("argument") => {
              buffer.push(format!("@ARG\n"));
            },
            Some("this") => {
              buffer.push(format!("@THIS\n"));
            },
            Some("that") => {
              buffer.push(format!("@THAT\n"));
            },
            _ => (),
          }
          buffer.push(format!("A=M+D\n"));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        }
      },
      CommandType::POP => {
        if segment.to_str() == Some("temp") {
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M-1\n"));
          buffer.push(format!("A=M\n"));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@{}\n", index + 5));
          buffer.push(format!("M=D\n"));
        } else if segment.to_str() == Some("pointer") {
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M-1\n"));
          buffer.push(format!("A=M\n"));
          buffer.push(format!("D=M\n"));
          match index {
            0 => {
              buffer.push(format!("@THIS\n"));
            },
            1 => {
              buffer.push(format!("@THAT\n"));
            },
            _ => (),
          }
          buffer.push(format!("M=D\n"));
        } else if segment.to_str() == Some("static") {
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M-1\n"));
          buffer.push(format!("A=M\n"));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@STATIC{}\n", index));
          buffer.push(format!("M=D\n"));
        } else {
          buffer.push(format!("@{}\n", index));
          buffer.push(format!("D=A\n"));
          match segment.to_str() {
            Some("local") => {
              buffer.push(format!("@LCL\n"));
            },
            Some("argument") => {
              buffer.push(format!("@ARG\n"));
            },
            Some("this") => {
              buffer.push(format!("@THIS\n"));
            },
            Some("that") => {
              buffer.push(format!("@THAT\n"));
            },
            _ => (),
          }
          buffer.push(format!("A=M+D\n"));
          buffer.push(format!("D=A\n"));
          buffer.push(format!("@R13\n"));
          buffer.push(format!("M=D\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M-1\n"));
          buffer.push(format!("A=M\n"));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@R13\n"));
          buffer.push(format!("A=M\n"));
          buffer.push(format!("M=D\n"));
        }
      },
      CommandType::LABEL => {
        buffer.push(format!("({})\n", segment.to_str().unwrap_or("")));
      },
      CommandType::GOTO => {
        buffer.push(format!("@{}\n", segment.to_str().unwrap_or("")));
        buffer.push(format!("0;JMP\n"));
      },
      CommandType::IFGOTO => {
        buffer.push(format!("@SP\n"));
        buffer.push(format!("M=M-1\n"));
        buffer.push(format!("A=M\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("@{}\n", segment.to_str().unwrap_or("")));
        buffer.push(format!("D;JNE\n"));
      },
      CommandType::CALL => {
        buffer.push(format!(
          "@{}$ret.{}\n",
          self.context_name.last().unwrap_or(&"".to_string()),
          self.context_index.last().unwrap()
        ));
        buffer.push(format!("M=M+1\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("M=A\n"));

        let addresses = vec![
          "@LCL".to_string(),
          "@ARG".to_string(),
          "@THIS".to_string(),
          "@THAT".to_string()
        ];

        for address in addresses {
          buffer.push(format!("{}\n", address));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=D\n"));
        }

        buffer.push(format!("@5\n",));
        buffer.push(format!("D=A\n"));
        buffer.push(format!("@{}\n", index));
        buffer.push(format!("D=D+A\n"));
        buffer.push(format!("@SP\n"));
        buffer.push(format!("D=M-D\n"));
        buffer.push(format!("@ARG\n"));
        buffer.push(format!("M=D\n"));

        buffer.push(format!("@SP\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("@LCL\n"));
        buffer.push(format!("M=D\n"));

        buffer.push(format!("@{}\n", segment.to_str().unwrap_or("")));
        buffer.push(format!("0;JMP\n"));
        buffer.push(format!(
          "({}$ret.{})\n",
          self.context_name.last().unwrap_or(&"".to_string()),
          self.context_index.last().unwrap()
        ));

      },
      CommandType::FUNCTION => {
        self.push_context(segment.to_str().unwrap_or("").to_string());

        buffer.push(format!("({})\n", segment.to_str().unwrap_or("")));

        for _ in (0 as i16)..index {
          buffer.push(format!("@SP\n"));
          buffer.push(format!("M=M+1\n"));
          buffer.push(format!("A=M-1\n"));
          buffer.push(format!("M=0\n"));
        }
      },
      CommandType::RETURN => {
        buffer.push(format!("@LCL\n"));
        buffer.push(format!("D=A\n"));
        buffer.push(format!("@R13\n"));
        buffer.push(format!("M=D\n"));

        buffer.push(format!("@5\n"));
        buffer.push(format!("D=A\n"));
        buffer.push(format!("@R13\n"));
        buffer.push(format!("A=M\n"));
        buffer.push(format!("A=A-D\n"));

        buffer.push(format!("@SP\n"));
        buffer.push(format!("A=M-1\n"));
        buffer.push(format!("D=M\n"));
        buffer.push(format!("@ARG\n"));
        buffer.push(format!("M=D\n"));
        buffer.push(format!("D=A+1\n"));
        buffer.push(format!("@SP\n"));
        buffer.push(format!("M=D\n"));

        let mut index = 1;
        let addresses = vec![
          "@THAT".to_string(),
          "@THIS".to_string(),
          "@ARG".to_string(),
          "@LCL".to_string()
        ];

        for address in addresses {
          buffer.push(format!("@{}\n", index));
          buffer.push(format!("D=A\n"));
          buffer.push(format!("@R13\n"));
          buffer.push(format!("A=M-D\n"));
          buffer.push(format!("D=M\n"));
          buffer.push(format!("{}\n", address));
          buffer.push(format!("M=D\n"));

          index = index + 1;
        }

        match self.context_name.last() {
          Some(x) => {
            buffer.push(format!(
              "@{}$ret.{}\n",
              x,
              self.context_index.last().unwrap()
            ));
            buffer.push(format!("0;JMP\n"));
          },
          None => {
            buffer.push(format!("(MAIN_END)\n"));
            buffer.push(format!("@MAIN_END\n"));
            buffer.push(format!("0;JMP\n"));
          },
        };
        self.pop_context();
      },
      _ => return Err(format!("(some command) with segment {} not implemented", segment.to_str().unwrap_or(""))),
    }

    if let Some(buffer) = buffer.to_str() {
      match self.out_file.write(buffer.as_bytes()) {
        Err(e) => panic!("Error: {}", e),
        _ => ()
      }
    }
    return Ok(());
  }
}
