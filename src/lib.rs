pub mod helpers {
}

pub mod std {
    use std::fs::File;
    use std::ffi::OsString;
    use std::path::Path;
    use std::option::Option;

    pub struct Arguments {
        pub in_file: OsString,
        pub out_file: OsString,
    }

    impl Arguments {
        pub fn new(arguments: Vec<OsString>) -> Result<Arguments, String>{
            let in_file = Path::new(&arguments[0]);
            let out_file = Path::new(&arguments[1]);

            let in_extension = in_file.extension().unwrap();
            let out_extension = out_file.extension().unwrap();
            if in_extension == "vm" && out_extension == "asm" {
                return Ok(Arguments {
                  in_file: in_file.file_name().unwrap().to_os_string(),
                  out_file: out_file.file_name().unwrap().to_os_string(),
                })
            } else {
                return Err("How to use program text".to_string())
            }
        }
    }

    pub enum CommandType {
        ARITHMETIC,
        PUSH,
        POP,
        LABEL,
        GOTO,
        IF,
        FUNCTION,
        RETURN,
        CALL,
    }

    pub struct Parser {
        in_file: File,
    }

    impl Parser {
        pub fn new(path: &Path) -> Parser {
            return Parser {
                in_file: File::open(path).unwrap()
            }
        }

        pub fn commandType() -> CommandType {
            return CommandType::ARITHMETIC;
        }

        pub fn arg1() -> Option<OsString> {
            return Some(OsString::from("arg1"));
        }

        pub fn arg2() -> Option<OsString> {
            return Some(OsString::from("arg2"));
        }
    }

    pub struct CodeWriter {
        out_file: File,
    }

    impl CodeWriter {
        pub fn new(path: &Path) -> CodeWriter {
            return CodeWriter {
                out_file: File::open(path).unwrap(),
            }
        }

        pub fn writeArithmetic(command: String) {
            println!("Write Arithmetic");
        }

        pub fn writePushPop(command: CommandType, segment: String, index: u16) {

        }

        pub fn close() {

        }
    }
}