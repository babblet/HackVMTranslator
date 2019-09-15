use ::std::ffi::OsString;
use hack_vm_translator::arguments::Arguments;
use hack_vm_translator::parser::Parser;
use hack_vm_translator::codewriter::CodeWriter;
use ::std::path::Path;

fn main() {
    let environment_arguments: Vec<OsString> = std::env::args_os().collect();
    let arguments: Arguments = match Arguments::new(environment_arguments) {
        Ok(arguments_object) => arguments_object,
        Err(e) => panic!("Error when parsing arguments: {}", e),
    };

    //Should just get path in arguments
    let in_path: &Path = Path::new(&arguments.in_file);
    let parser: Parser = Parser::new(in_path);


    let out_path: &Path = Path::new(&arguments.out_file);
    let codewriter: CodeWriter = CodeWriter::new(out_path);

    loop {
        if !parser.has_more_commands() { break }
        parser.advance();


    }

    println!("Hello, world!");
}
