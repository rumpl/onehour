use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Nothing,
    Int(i64),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nothing => write!(f, "void"),
            Value::Int(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    SetVar(String, Value),
    GetVar(String),
    Push(Value),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    FuncCall(String),
    Ret,
    End,
}

#[derive(Debug)]
pub enum EngineError {
    MissingVariable(String),
    MismatchNumParams,
    MismatchType,
    UnknownCommand(String),
    EmptyStack,
}
