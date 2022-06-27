use std::collections::HashMap;

use crate::command::{Command, EngineError, Value};

pub struct Parser {}

#[derive(Default, Debug)]
pub struct Program {
    pub commands: Vec<Command>,
    pub functions: HashMap<String, usize>,
    pub labels: HashMap<String, usize>,
}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }

    fn parse_var_name(&self, input: &str) -> Result<String, EngineError> {
        Ok(input.into())
    }

    fn parse_int(&self, input: &str) -> Result<Value, EngineError> {
        let result = input.parse::<i64>();
        match result {
            Ok(x) => Ok(Value::Int(x)),
            Err(_) => Err(EngineError::MismatchType),
        }
    }

    fn parse_string(&self, input: &str) -> Result<Value, EngineError> {
        if input.starts_with('\"') && input.ends_with('\"') && input.len() > 1 {
            let inner = input[1..(input.len() - 1)].to_string();

            Ok(Value::String(inner))
        } else {
            Err(EngineError::MismatchType)
        }
    }

    fn parse_value(&self, input: &str) -> Result<Value, EngineError> {
        if input.starts_with('\"') && input.ends_with('"') && input.len() > 1 {
            self.parse_string(input)
        } else {
            self.parse_int(input)
        }
    }

    fn parse_set(&self, input: &[&str]) -> Result<Command, EngineError> {
        if input.len() != 3 {
            return Err(EngineError::MismatchNumParams);
        }

        let var_name = self.parse_var_name(input[1])?;
        let value = self.parse_value(input[2])?;

        Ok(Command::SetVar(var_name, value))
    }

    fn parse_get(&self, input: &[&str]) -> Result<Command, EngineError> {
        if input.len() != 2 {
            return Err(EngineError::MismatchNumParams);
        }

        let var_name = self.parse_var_name(input[1])?;

        Ok(Command::GetVar(var_name))
    }

    fn parse_push(&self, input: &[&str]) -> Result<Command, EngineError> {
        if input.len() != 2 {
            return Err(EngineError::MismatchNumParams);
        }

        let var_name = self.parse_value(input[1])?;

        Ok(Command::Push(var_name))
    }

    fn parse_func_call(&self, input: &[&str]) -> Result<Command, EngineError> {
        if input.len() != 2 {
            return Err(EngineError::MismatchNumParams);
        }

        Ok(Command::FuncCall(input[1].into()))
    }

    pub fn parse(&self, input: &str) -> Result<Program, EngineError> {
        let mut output = vec![];
        let mut functions: HashMap<String, usize> = HashMap::new();

        let mut labels: HashMap<String, usize> = HashMap::new();

        for line in input.lines() {
            let command: Vec<_> = line.split_ascii_whitespace().collect();

            match command.get(0) {
                Some(x) if x.contains(':') => {
                    if let Some(label) = x.strip_suffix(':') {
                        labels.insert(label.into(), output.len());
                    } else {
                        return Err(EngineError::UnknownCommand(x.to_string()));
                    }
                }
                Some(x) if *x == "set" => {
                    output.push(self.parse_set(&command)?);
                }
                Some(x) if *x == "get" => {
                    output.push(self.parse_get(&command)?);
                }
                Some(x) if *x == "push" => {
                    output.push(self.parse_push(&command)?);
                }
                Some(x) if *x == "pop" => {
                    output.push(Command::Pop);
                }
                Some(x) if *x == "add" => {
                    output.push(Command::Add);
                }
                Some(x) if *x == "mul" => {
                    output.push(Command::Mul);
                }
                Some(x) if *x == "sub" => {
                    output.push(Command::Sub);
                }
                Some(x) if *x == "div" => {
                    output.push(Command::Div);
                }
                Some(x) if *x == "func" => {
                    functions.insert(command[1].into(), output.len());
                }
                Some(x) if *x == "ret" => output.push(Command::Ret),
                Some(x) if *x == "end" => output.push(Command::End),
                Some(x) if *x == "call" => output.push(self.parse_func_call(&command)?),
                Some(x) if *x == "cmp" => output.push(Command::Cmp),
                Some(x) if *x == "jz" => output.push(Command::Jz(command[1].into())),
                Some(x) if *x == "jp" => output.push(Command::Jp(command[1].into())),
                Some(x) if *x == "jn" => output.push(Command::Jn(command[1].into())),
                Some(name) => return Err(EngineError::UnknownCommand(name.to_string())),
                None => {}
            }
        }

        Ok(Program {
            commands: output,
            functions,
            labels,
        })
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}
