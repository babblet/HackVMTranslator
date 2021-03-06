use super::commandtype::CommandType;
use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use std::path::Path;

pub struct FileParser {
  pub file_name: OsString,
  pub arg1: OsString,
  pub arg2: i16,
  pub command_is_arithmetic: bool,
  pub current_command_type: CommandType,
  pub current_unparsed_line: usize,
  pub in_file_lines: Vec<OsString>,
}

impl FileParser {
  pub fn new(path: &Path) -> FileParser {
    println!("DEBUG: path = {}", path.to_str().unwrap());

    let mut in_file: File = match File::open(path) {
      Ok(file) => file,
      Err(e) => panic!("Failed to load {}: {}", path.to_str().unwrap(), e)
    };

    let file_name = match path.file_name() {
      Some(file_name) => file_name.to_os_string(),
      None => OsString::from(""),
    };

    let mut buffer = String::new();
    match in_file.read_to_string(&mut buffer) {
      Ok(size) => println!("Read {} bytes from {}", size, path.to_str().unwrap()),
      Err(e) => println!("Was unable to read from file: {}", e),
    };

    let lines: Vec<OsString> = buffer.lines().map(|x| OsString::from(x)).collect();

    return FileParser {
      file_name: file_name,
      arg1: OsString::new(),
      arg2: 0,
      command_is_arithmetic: false,
      current_command_type: CommandType::NULL,
      current_unparsed_line: 0,
      in_file_lines: lines,
    }
  }

  pub fn has_more_commands(&self) -> bool {
    if self.in_file_lines.len() > self.current_unparsed_line { true } else { false }
  }

  pub fn advance(&mut self) {
    let line: String = String::from(self.in_file_lines[self.current_unparsed_line].to_str().unwrap());
    if line.trim().is_empty() || line.split("//").next().unwrap_or("").is_empty() {
      self.in_file_lines.remove(self.current_unparsed_line);
      self.advance();
    } else {
      self.current_unparsed_line = self.current_unparsed_line + 1;
      let mut _split = line.split('\t');
      let mut split = _split.next().unwrap().split(' ');
      match split.next() {
        Some(_x) => {
          let x = _x.trim();
          println!("command = {}", x);
          if      x == "if-goto"  { self.current_command_type = CommandType::IFGOTO;   }
          else if x == "goto"     { self.current_command_type = CommandType::GOTO;     }
          else if x == "push"     { self.current_command_type = CommandType::PUSH;     }
          else if x == "pop"      { self.current_command_type = CommandType::POP;      }
          else if x == "call"     { self.current_command_type = CommandType::CALL;     }
          else if x == "label"    { self.current_command_type = CommandType::LABEL;    }
          else if x == "return"   { self.current_command_type = CommandType::RETURN;   }
          else if x == "function" { self.current_command_type = CommandType::FUNCTION; }
          else {
            self.command_is_arithmetic = true;
            if      x == "add" { self.current_command_type = CommandType::ADD; }
            else if x == "sub" { self.current_command_type = CommandType::SUB; }
            else if x == "neg" { self.current_command_type = CommandType::NEG; }
            else if x == "eq"  { self.current_command_type = CommandType::EQ;  }
            else if x == "gt"  { self.current_command_type = CommandType::GT;  }
            else if x == "lt"  { self.current_command_type = CommandType::LT;  }
            else if x == "or"  { self.current_command_type = CommandType::OR;  }
            else if x == "and" { self.current_command_type = CommandType::AND; }
            else if x == "not" { self.current_command_type = CommandType::NOT; }
          }
        },
        None => return,
      };

      match split.next() {
        Some(_x) => {
          let x = _x.trim();
          println!("arg1 = {}", x);
          self.arg1 = OsString::from(x);
        },
        None => return,
      };

      match split.next() {
        Some(_x) => {
          let x = _x.trim();
          println!("arg2 = {}", x);
          self.arg2 = match x.parse::<i16>() {
            Ok(i) => i,
            _ => return,
          };
        },
        None => return,
      };
    }
  }

  pub fn command_type(&mut self) -> CommandType {
    self.current_command_type
  }
}
