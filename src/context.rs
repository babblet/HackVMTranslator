use std::ffi::OsString;
use std::collections::HashMap;

pub struct Context {
  pub functions: HashMap<OsString, u16>,
  pub function_context: Vec<OsString>
}

impl Context {
  pub fn new() -> Context {
      return Context {
        functions: HashMap::new(),
        function_context: Vec::new()
      }
  }

  pub fn get_function_value(&self, segment: &OsString) -> Option<u16> {
    return match self.functions.get(segment) {
      Some(value) => Some(*value),
      None => None,
    };
  }

  pub fn push_function(&mut self, segment: &OsString) {
    match self.get_function_value(segment) {
      Some(_) => { self.function_context.push(segment.to_os_string()); }
      _ => { self.functions.insert(segment.to_os_string(), 0); }
    };
  }

  pub fn add_call_context(&mut self, segment: &OsString) -> u16 {
    match self.get_function_value(segment) {
      Some(value) => { 
        self.functions.insert(segment.to_os_string(), value + 1);
        return value;
      }
      None => {
        self.functions.insert(segment.to_os_string(), 1);
        return 0;
      },
    };
  }
}
