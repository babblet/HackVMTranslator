#[cfg(test)]
mod arguments {
  use ::std::ffi::OsString;
  use hack_vm_translator::arguments::Arguments;

  #[test]
  fn new() {
    let mut args: Vec<OsString> = Vec::new();
    args.push(OsString::from("run"));
    args.push(OsString::from("in.vm"));
    args.push(OsString::from("out.asm"));
    let arguments: Arguments = match Arguments::new(args) {
      Ok(x) => x,
      Err(err) => panic!("Failed to get arguments: {}", err)
    };
    assert_eq!("in.vm", arguments.in_file);
    assert_eq!("out.asm", arguments.out_file);
  }
}