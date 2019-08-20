pub mod helpers {

}

pub mod std {
    use std::string::String;

    pub struct Arguments {
        pub in_file: String,
        pub out_file: String,
    }

    impl Arguments {
        fn get_extension(&mut self, s: String) -> Option<bool> {
            //let chars: Vec<char> = s.chars().collect();
            return Some(true);
        }

        pub fn new(arguments: Vec<String>) -> Result<Arguments, String>{
            let in_file: String = arguments[0];
            let out_file: String = arguments[1];
            if(Some(self.get_extension("Something"))) {
                Ok(
                    Arguments {
                        in_file: in_file,
                        out_file: out_file,
                    }
                );
            } else {
                Err("Argument error");
            }
        }
    }

    //pub struct Parser {

    //}
}