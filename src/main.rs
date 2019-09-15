use ::std::ffi::OsString;
use hack_vm_translator::arguments::Arguments;
use hack_vm_translator::parser::Parser;
use hack_vm_translator::codewriter::CodeWriter;
use hack_vm_translator::commandtype::CommandType;
use ::std::path::Path;

fn main() {
    let environment_arguments: Vec<OsString> = std::env::args_os().collect();
    let arguments: Arguments = match Arguments::new(environment_arguments) {
        Ok(arguments_object) => arguments_object,
        Err(e) => panic!("Error when parsing arguments: {}", e),
    };

    //Should just get path in arguments
    let in_path: &Path = Path::new(&arguments.in_file);
    let mut parser: Parser = Parser::new(in_path);


    let out_path: &Path = Path::new(&arguments.out_file);
    let codewriter: CodeWriter = CodeWriter::new(out_path);

    loop {
        if !parser.has_more_commands() { break }
        parser.advance();

        let command_type: CommandType = parser.command_type();
        if command_type == CommandType::PUSH || command_type == CommandType::POP {
            codewriter.write_push_pop(command_type, &parser.arg1, &parser.arg2);
        } else {
            codewriter.write_arithmetic(&parser.arg1);
        }
        //parse the line
        //take the args form the line and write them in asm to file with CodeWriter

    }

    println!("Hello, world!");
}
