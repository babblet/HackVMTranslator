use super::commandtype::CommandType;
use std::fs::File;
use std::ffi::OsString;
use std::option::Option;
use std::path::Path;

pub struct Parser {
    in_file: File,
}

impl Parser {
    pub fn new(path: &Path) -> Parser {
        return Parser {
            in_file: File::open(path).unwrap()
        }
    }

    pub fn command_type() -> CommandType {
        return CommandType::ARITHMETIC;
    }

    pub fn arg1() -> Option<OsString> {
        return Some(OsString::from("arg1"));
    }

    pub fn arg2() -> Option<OsString> {
        return Some(OsString::from("arg2"));
    }
}