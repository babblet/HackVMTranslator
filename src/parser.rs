use super::commandtype::CommandType;
use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use std::option::Option;
use std::path::Path;

pub struct Parser {
    pub arg1: OsString,
    pub arg2: u16,
    pub current_command_type: CommandType,
    pub current_unparsed_line: usize,
    pub in_file_lines: Vec<OsString>,
}

impl Parser {
    pub fn new(path: &Path) -> Parser {
        let mut in_file: File = match File::open(path) {
            Ok(file) => file,
            Err(e) => panic!("Failed to load {}: {}", path.to_str().unwrap(), e)
        };

        let mut buffer = String::new();
        match in_file.read_to_string(&mut buffer) {
            Ok(size) => println!("Read {} bytes from {}", size, path.to_str().unwrap()),
            Err(e) => println!("Was unable to read from file: {}", e),
        };

        let lines: Vec<OsString> = buffer.lines().map(|x| OsString::from(x)).collect();

        return Parser {
            arg1: OsString::new(),
            arg2: 0,
            current_command_type: CommandType::NULL,
            current_unparsed_line: 0,
            in_file_lines: lines,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        if self.in_file_lines.len() >= self.current_unparsed_line { true } else { false }
    }

    pub fn advance(&mut self) {
        let line: String = String::from(self.in_file_lines[self.current_unparsed_line].to_str().unwrap());
        let mut start_of_comment: bool = false;
        let mut is_comment: bool = false;
        let mut buffer: String = String::new();
        let mut arg1: String = String::new();
        let mut arg2: u16 = 0;
        for c in line.chars() {
            if c == '/' {
                if start_of_comment { self.in_file_lines.remove(self.current_unparsed_line);
                  is_comment = true;
                  break;
                } else {
                    start_of_comment = true;
                }
            } else {
                if c == '\n' { break; }
                else if c == ' ' {
                    if !arg1.is_empty() {
                        arg2 = buffer.parse::<u16>().unwrap();
                        buffer = "".to_string();
                    } else {
                        arg1 = buffer.to_string();
                        buffer = "".to_string();
                    }
                } else {
                    buffer.push(c);
                }
            }
        }

        if is_comment {
            self.advance();
        } else {
            self.arg1 = OsString::from(arg1);
            self.arg2 = arg2;
            self.current_unparsed_line = self.current_unparsed_line + 1;
        }
    }

    pub fn command_type(&mut self) -> CommandType {
        self.current_command_type
    }
}