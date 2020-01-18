use ::std::path::Path;
use ::std::ffi::OsString;

use hack_vm_translator::arguments::Arguments;
use hack_vm_translator::parser::FileParser;
use hack_vm_translator::codewriter::CodeWriter;
use hack_vm_translator::commandtype::CommandType;
use hack_vm_translator::context::Context;

fn main() {
  let environment_arguments: Vec<OsString> = std::env::args_os().collect();
  let arguments: Arguments = match Arguments::new(environment_arguments) {
    Ok(arguments_object) => arguments_object,
    Err(e) => {
      println!("Argument error: {}", e);
      return ();
    }
  };

  let mut file_parsers: Vec<FileParser> = Vec::new();
  for input_path in arguments.input {
    let path: &Path = Path::new(&input_path);
    file_parsers.push(FileParser::new(path));
  } 

  let output_path: &Path = Path::new(&arguments.output);
  let mut codewriter: CodeWriter = CodeWriter::new(output_path);

  let mut context = Context::new();

  for mut parser in file_parsers {
    loop {
      if !parser.has_more_commands() { break }
      println!("Has more commands! current line = {}", parser.current_unparsed_line);
      parser.advance();

      let command_type: CommandType = parser.command_type();

      if command_type == CommandType::FUNCTION && parser.arg1 == "Sys.init" {
        //Bootstrap Sys.init
        match codewriter.write(CommandType::BOOTSTRAP, &OsString::from("Sys.init"), 0, &parser.file_name, &mut context) {
          Err(err) => {
            println!("Codewriter error: {}", err);
            return ();
          },
          _ => ()
        }
        //Call Sys.init
        match codewriter.write(CommandType::CALL, &OsString::from("Sys.init"), 0, &parser.file_name, &mut context) {
          Err(err) => {
            println!("Codewriter error: {}", err);
            return ();
          },
          _ => ()
        }
      }

      match codewriter.write(command_type, &parser.arg1, parser.arg2, &parser.file_name, &mut context) {
        Err(err) => {
          println!("Codewriter error: {}", err);
          return ();
        },
        _ => ()
      }
    }
  }
}
