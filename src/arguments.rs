use std::fs;
use std::fs::read_dir;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

fn get_directory_files(path: &Path) -> Option<Vec<OsString>> {
  match read_dir(path) {
    Ok(entries) => {
      let mut files: Vec<OsString> = Vec::new();
      let mut sys_path: OsString = OsString::from("");
      for entry in entries {
        match entry {
          Ok(file) => {
            println!("Got file");
            let path: PathBuf = file.path();
            let string_path: OsString = OsString::from(path.to_str().unwrap_or(""));
            if Path::new(string_path.to_str().unwrap_or("")).file_name() == Some(&OsStr::new("Sys.vm")) {
              sys_path = string_path;
            } else if path.extension().unwrap_or_default().to_str() == Some("vm") {
              files.push(string_path);
            }
          },
          Err(e) => println!("{}", e),
        }
      }
      if !sys_path.is_empty() {
        files.insert(0, sys_path);
      }

      if files.len() != 0 {
        return Some(files);
      }
    },
    Err(e) => {
      println!("{}", e);
    },
  };
  return None;
}

pub struct Arguments {
  pub input: Vec<OsString>,
  pub output: OsString,
}

impl Arguments {
  pub fn new(arguments: Vec<OsString>) -> Result<Arguments, String>{
    let _input = Path::new(&arguments[1]);
    let _output = Path::new(&arguments[2]);

    let output = match _output.extension() {
      Some(extension) => {
        OsString::from(_output.to_str().unwrap())
      },
      None => return Err("Output file wrong format!".to_string()),
    };

    if _input.is_file() {
      match _input.extension() {
        Some(extension) => {
          if extension == "vm" {
            return Ok(Arguments {
              input: vec![OsString::from(_input.to_str().unwrap())],
              output: output,
            })
          } else {
            return Err(format!("Input has incorrect file format! Expecting <file>.vm instead of <file>.{}", extension.to_str().unwrap_or_default()));
          }
        },
        None => return Err("Input has incorrent file format! Expecting <file>.vm".to_string()),
      }
    } else if _input.is_dir() {
      match get_directory_files(_input) {
        Some(files) => {
          return Ok(Arguments {
            input: files,
            output: OsString::from(_output.to_str().unwrap()),
          });
        },
        None => return Err("Could not find input files in given directory! Check that file extensions are correct and that read permissions are granted.".to_string()),
      };
    } else {
      return Err("Input is neither a file nor a directory!".to_string());
    };
  }
}