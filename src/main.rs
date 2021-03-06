pub mod command;
pub mod eval;
pub mod oh;
pub mod parser;

use command::EngineError;
use eval::Evaluator;
use oh::parser::Parser as OhParser;
use parser::Parser;

fn main() -> Result<(), EngineError> {
    let mut oh_parser = OhParser::new();
    let contents = std::fs::read_to_string("./samples/oh/tokens.oh").unwrap();
    let result = oh_parser.parse(&contents)?;
    println!("{:?}", result);

    for file in std::env::args().skip(1) {
        let contents = std::fs::read_to_string(file).unwrap();
        let parser = Parser::new();
        let commands = parser.parse(&contents)?;
        let mut eval = Evaluator::new();
        let result = eval.evaluate(&commands)?;

        println!("Result -> {}", result);
    }

    Ok(())
}

#[test]
fn test1() -> Result<(), EngineError> {
    use command::{Command, Value};
    use parser::Program;
    use std::collections::HashMap;

    let program = Program {
        commands: vec![
            Command::SetVar("a".into(), Value::Int(100)),
            Command::GetVar("a".into()),
        ],
        functions: HashMap::from([(String::from("main"), 0)]),
        labels: Default::default(),
    };

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&program)?;

    assert_eq!(result, Value::Int(100));
    Ok(())
}

#[test]
fn test2() -> Result<(), EngineError> {
    use command::Value;
    let intput = "func main\nset x 30\nget x\nend";
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
    let intput = "func main\npush 5\npush 10\nsub\npop\nend";
    let parser = Parser::new();
    let commands = parser.parse(intput)?;

    let mut evaluator = Evaluator::new();

    let result = evaluator.evaluate(&commands)?;

    assert_eq!(result, Value::Int(5));
    Ok(())
}
