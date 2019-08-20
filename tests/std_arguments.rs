#[cfg(test)]
mod tests {
    use std::env;
    use hack_vm_translator::std::Arguments
      // Note this useful idiom: importing names from outer (for mod tests) scope.

    #[test]
    fn test_expected() {
        //std::env::args().collect();
        let args = vec!["in.vm", "out.asm"];
        assert_eq!(Arguments.new(args), Ok(Arguments));
    }
}