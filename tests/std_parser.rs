#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::ffi::OsString;
    use hack_vm_translator::parser::Parser;

    #[test]
    fn test_parser_new() {
      let test_file_path: &Path = Path::new("tests/std_parser/StackArithmetic/SimpleAdd/SimpleAdd.vm");
      let parser: Parser = Parser::new(test_file_path);
      assert_eq!(parser.current_unparsed_line, 0);
      assert_eq!(parser.arg1, OsString::new());
      assert_eq!(parser.arg2, OsString::new());
    }

    #[test]
    fn test_has_more_commands() {
      let test_file_path: &Path = Path::new("tests/std_parser/StackArithmetic/SimpleAdd/SimpleAdd.vm");
      let parser: Parser = Parser::new(test_file_path);
      assert_eq!(parser.has_more_commands(), true);
    }
}