#[cfg(test)]
mod codewriter {
    use std::path::Path;
    use hack_vm_translator::codewriter::CodeWriter;
    #[test]
    fn new() {
      let test_out_file_path: &Path = Path::new("tests/std_codewriter/test_out.asm");
      let codewriter = CodeWriter::new(test_out_file_path);
      //
    }
}
