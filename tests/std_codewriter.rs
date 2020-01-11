
#[cfg(test)]
mod codewriter {
  use std::fs::File;
  use std::io::Read;
  use std::path::Path;
  use hack_vm_translator::codewriter::CodeWriter;

  #[test]
  fn new() {
    let test_out_file_path: &Path = Path::new("tests/std_codewriter/test_out.asm");
    let codewriter = CodeWriter::new(test_out_file_path);

    let mut in_file: File = match File::open("tests/std_codewriter/test_out.asm") {
      Ok(file) => file,
      _ => panic!("Test failed when uwraping file"),
    };

    let mut buffer = String::new();
    match in_file.read_to_string(&mut buffer) {
      Ok(size) => println!("Read {} bytes from tests/std_codewriter/test_out.asm", size),
      Err(e) => println!("Was unable to read from file: {}", e),
    };
    let mut lines = buffer.lines();
    assert_eq!(lines.next(), Some("@256"));
    assert_eq!(lines.next(), Some("D=A"));
    assert_eq!(lines.next(), Some("@0"));
    assert_eq!(lines.next(), Some("M=D"));

    assert_eq!(lines.next(), Some("@2048"));
    assert_eq!(lines.next(), Some("D=A"));
    assert_eq!(lines.next(), Some("@1"));
    assert_eq!(lines.next(), Some("M=D"));
  }
}
