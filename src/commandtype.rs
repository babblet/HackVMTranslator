#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum CommandType {
    NULL,
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    RETURN,
    CALL,
}