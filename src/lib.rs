pub mod helpers {
}

pub mod std {
    use std::ffi::OsString;
    use std::path::Path;

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
}