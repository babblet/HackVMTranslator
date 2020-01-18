#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CommandType {
  NULL,
  PUSH,
  POP,
  LABEL,
  GOTO,
  IFGOTO,
  FUNCTION,
  RETURN,
  BOOTSTRAP,
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
