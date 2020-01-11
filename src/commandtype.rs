#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CommandType {
  NULL,
  PUSH,
  POP,
  LABEL,
  GOTO,
  IF,
  FUNCTION,
  RETURN,
  CALL,
  ADD,
  SUB,
  NEG,
  EQ,
  GT,
  LT,
  OR,
  AND,
  NOT
}
