pub mod command;
pub mod eval;
pub mod parser;

use command::EngineError;
use eval::Evaluator;
use parser::Parser;

fn main() -> Result<(), EngineError> {
    for file in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(file).unwrap();
        let parser = Parser::new();
        let commands = parser.parse(&contents)?;
        let mut eval = Evaluator::new();
        let result = eval.evaluate(&commands)?;

        println!("{}", result);
    }

    Ok(())
}

#[test]
fn test1() -> Result<(), EngineError> {
    use command::{Command, Value};
    use parser::Program;

    let program = Program {
        commands: vec![
            Command::SetVar("a".into(), Value::Int(100)),
            Command::GetVar("a".into()),
        ],
        functions: Default::default(),
    };

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&program)?;

    assert_eq!(result, Value::Int(100));
    Ok(())
}

#[test]
fn test2() -> Result<(), EngineError> {
    use command::Value;
    let intput = "set x 30\nget x";
    let parser = Parser::new();
    let commands = parser.parse(intput)?;

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(30));
    Ok(())
}

#[test]
fn test_sub() -> Result<(), EngineError> {
    use command::Value;
    let intput = "push 5\npush 10\nsub\npop";
    let parser = Parser::new();
    let commands = parser.parse(intput)?;

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(5));
    Ok(())
}
