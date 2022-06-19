use std::collections::HashMap;

use crate::command::{Command, EngineError, Value};
use crate::parser::Program;

pub struct Evaluator {
    vars: HashMap<String, Value>,
    stack: Vec<Value>,
    pc: usize,
    pc_stack: Vec<usize>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            stack: vec![],
            pc: 0,
            pc_stack: vec![],
        }
    }

    fn push(&mut self, value: Value) -> Result<(), EngineError> {
        self.stack.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<Value, EngineError> {
        let result = self.stack.pop();
        match result {
            Some(x) => Ok(x),
            None => Err(EngineError::EmptyStack),
        }
    }

    fn add(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            _ => Err(EngineError::MismatchType),
        }
    }

    fn sub(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            _ => Err(EngineError::MismatchType),
        }
    }

    fn mul(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            _ => Err(EngineError::MismatchType),
        }
    }

    fn div(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Int(a / b)),
            _ => Err(EngineError::MismatchType),
        }
    }

    fn cmp(&self, lhs: Value, rhs: Value) -> Result<Value, EngineError> {
        match (lhs, rhs) {
            (Value::Int(a), Value::Int(b)) => {
                if a > b {
                    return Ok(Value::Int(-1));
                }
                if a == b {
                    return Ok(Value::Int(0));
                }
                Ok(Value::Int(1))
            }
            _ => Err(EngineError::MismatchType),
        }
    }

    pub fn evaluate(&mut self, program: &Program) -> Result<Value, EngineError> {
        self.pc = program.functions["main"];
        let mut output = Ok(Value::Nothing);
        let mut update_pc: bool;

        loop {
            update_pc = true;
            if self.pc >= program.commands.len() {
                break;
            }

            let command = &program.commands[self.pc];

            match command {
                Command::SetVar(name, value) => {
                    self.vars.insert(name.into(), value.clone());
                }
                Command::GetVar(name) => match self.vars.get(name) {
                    Some(value) => output = Ok(value.clone()),
                    None => return Err(EngineError::MissingVariable(name.into())),
                },
                Command::Push(value) => {
                    self.push(value.clone())?;
                }
                Command::Pop => {
                    output = self.pop();
                }
                Command::Add => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.add(lhs, rhs)?;
                    self.stack.push(result);
                }
                Command::Mul => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.mul(lhs, rhs)?;
                    self.stack.push(result);
                }
                Command::Sub => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.sub(lhs, rhs)?;
                    self.stack.push(result);
                }
                Command::Div => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.div(lhs, rhs)?;
                    self.stack.push(result);
                }
                Command::FuncCall(name) => {
                    if name == "print" {
                        let value = self.pop()?;
                        println!("{}", value);
                    } else {
                        self.pc_stack.push(self.pc + 1);
                        self.pc = program.functions[name];
                        update_pc = false;
                    }
                }
                Command::Ret => {
                    if let Some(pc) = self.pc_stack.pop() {
                        self.pc = pc;
                        update_pc = false;
                    } else {
                        return Err(EngineError::EmptyStack);
                    }
                }
                Command::End => {
                    break;
                }
                Command::Cmp => {
                    let lhs = self.pop()?;
                    let rhs = self.pop()?;

                    let result = self.cmp(lhs, rhs)?;
                    self.stack.push(result);
                }
                Command::Jn(label) => {
                    let value = self.pop()?;
                    match value {
                        Value::Int(x) => {
                            if x < 0 {
                                self.pc = program.labels[label];
                                update_pc = false;
                            }
                        }
                        Value::Nothing => return Err(EngineError::EmptyStack),
                        Value::String(_) => return Err(EngineError::EmptyStack),
                    }
                }
                Command::Jp(label) => {
                    let value = self.pop()?;
                    match value {
                        Value::Int(x) => {
                            if x > 0 {
                                self.pc = program.labels[label];
                                update_pc = false;
                            }
                        }
                        Value::Nothing => return Err(EngineError::EmptyStack),
                        Value::String(_) => return Err(EngineError::EmptyStack),
                    }
                }
                Command::Jz(label) => {
                    let value = self.pop()?;
                    match value {
                        Value::Int(x) => {
                            if x == 0 {
                                self.pc = program.labels[label];
                                update_pc = false;
                            }
                        }
                        Value::Nothing => return Err(EngineError::EmptyStack),
                        Value::String(_) => return Err(EngineError::EmptyStack),
                    }
                }
            }

            if update_pc {
                self.pc += 1;
            }
        }

        output
    }
}
