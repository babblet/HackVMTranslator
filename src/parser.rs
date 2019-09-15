use super::commandtype::CommandType;
use std::fs::File;
use std::io::Read;
use std::ffi::OsString;
use std::option::Option;
use std::path::Path;

pub struct Parser {
    pub arg1: OsString,
    pub arg2: OsString,
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
            arg2: OsString::new(),
            current_unparsed_line: 0,
            in_file_lines: lines,
        }
    }

    pub fn has_more_commands(&self) -> bool {
        if self.in_file_lines.len() >= self.current_unparsed_line { true } else { false }
    }

    pub fn advance(&self) {
    }

    pub fn command_type(&self) -> CommandType {
        return CommandType::ARITHMETIC;
    }

    pub fn arg1(&self) -> Option<OsString> {

        return Some(OsString::from("arg1"));
    }

    pub fn arg2(&self) -> Option<OsString> {
        return Some(OsString::from("arg2"));
    }
}