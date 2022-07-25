use std::{collections::VecDeque, io::Write};

use risp::{Error, Interpreter, Value};

fn prompt(name: &str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_string();
}

pub fn main() {
    let mut arguments: VecDeque<String> = std::env::args().collect();
    arguments.pop_front();

    match arguments.len() {
        0 => {
            println!("Welcome to RISP 🎉\n");
            let mut interpreter = Interpreter::new();

            loop {
                let expression = prompt("> ");

                if expression.is_empty() {
                    continue;
                }

                match parse_and_evaluate(&mut interpreter, &expression) {
                    Ok(value) => println!("{}", value.to_display_string()),
                    Err(error) => println!("{:?}", error),
                }
            }
        }

        _ => {
            println!("risp-lang\n\nUsage: risp [filename]");
        }
    }
}

fn parse_and_evaluate(interpreter: &mut Interpreter, input: &str) -> Result<Value, Error> {
    let tokens = risp::tokenize(input);
    let expression = risp::parse_node(&mut tokens.into_iter().peekable())?;

    return interpreter.evaluate(&expression);
}
