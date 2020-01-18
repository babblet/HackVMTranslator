#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct _Address {
  pub stack: i16,
  pub local: i16,
  pub argument: i16,
  pub this: i16,
  pub that: i16,
}

pub const ADDRESS: _Address = _Address {
  stack: 256,
  local: 400,
  argument: 500,
  this: 3000,
  that: 3100,    
};