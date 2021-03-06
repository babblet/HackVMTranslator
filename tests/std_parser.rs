#[cfg(test)]
mod parser {
  use std::path::Path;
  use std::ffi::OsString;
  use hack_vm_translator::parser::Parser;
  use hack_vm_translator::commandtype::CommandType;

  #[test]
  fn new() {
    let test_file_path: &Path = Path::new("tests/test_files/StackArithmetic/SimpleAdd/SimpleAdd.vm");
    let mut parser: Parser = Parser::new(test_file_path);
    assert_eq!(parser.current_unparsed_line, 0);
    assert_eq!(parser.command_type(), CommandType::NULL);
    assert_eq!(parser.arg1, OsString::new());
    assert_eq!(parser.arg2, 0);
    assert_eq!(parser.in_file_lines[0], "// This file is part of www.nand2tetris.org")
  }

  #[test]
  fn has_more_commands() {
    let test_file_path: &Path = Path::new("tests/test_files/StackArithmetic/SimpleAdd/SimpleAdd.vm");
    let parser: Parser = Parser::new(test_file_path);
    assert_eq!(parser.has_more_commands(), true);
  }

  #[test]
  fn advance() {
    let test_file_path: &Path = Path::new("tests/test_files/StackArithmetic/SimpleAdd/SimpleAdd.vm");
    let mut parser: Parser = Parser::new(test_file_path);
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), true);
    assert_eq!(parser.command_type(), CommandType::PUSH);
    assert_eq!(parser.arg1, "constant");
    assert_eq!(parser.arg2, 7);
    assert_eq!(parser.command_is_arithmetic, false);
  }

  #[test]
  fn double_advance() {
    let test_file_path: &Path = Path::new("tests/test_files/StackArithmetic/SimpleAdd/SimpleAdd.vm");
    let mut parser: Parser = Parser::new(test_file_path);
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), true);
    assert_eq!(parser.command_type(), CommandType::PUSH);
    assert_eq!(parser.arg1, "constant");
    assert_eq!(parser.arg2, 8);
    assert_eq!(parser.command_is_arithmetic, false);
  }

  #[test]
  fn triple_advance() {
    let test_file_path: &Path = Path::new("tests/test_files/StackArithmetic/SimpleAdd/SimpleAdd.vm");
    let mut parser: Parser = Parser::new(test_file_path);
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), true);
    parser.advance();
    assert_eq!(parser.has_more_commands(), false);
    assert_eq!(parser.command_type(), CommandType::ADD);
    assert_eq!(parser.command_is_arithmetic, true);

    //These are never used when command type is arithmetic, but for sanity we do a check on it
    assert_eq!(parser.arg1, "constant");
    assert_eq!(parser.arg2, 8);
  }
}
