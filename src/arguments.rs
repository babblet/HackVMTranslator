use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
pub struct Arguments {
    pub in_file: OsString,
    pub out_file: OsString,
}

impl Arguments {
    pub fn new(arguments: Vec<OsString>) -> Result<Arguments, String>{
        let in_file = Path::new(&arguments[1]);
        let out_file = Path::new(&arguments[2]);

        let in_extension = in_file.extension().unwrap_or(&OsStr::new(""));
        println!("in_extension: {}", in_extension.to_str().unwrap());
        let out_extension = out_file.extension().unwrap_or(&OsStr::new(""));
        println!("out_extension: {}", out_extension.to_str().unwrap());
        if in_extension == "vm" && out_extension == "asm" {
            return Ok(Arguments {
              in_file: OsString::from(in_file.to_str().unwrap()),
              out_file: OsString::from(out_file.to_str().unwrap()),
            })
        } else {
            return Err("How to use program text".to_string())
        }
    }
}