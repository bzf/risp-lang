use std::io::Write;

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
    println!("Welcome to RISP 🎉\n");
    let mut interpreter = Interpreter::new();

    loop {
        let expression = prompt("> ");

        if expression.is_empty() {
            continue;
        }

        match parse_and_evaluate(&mut interpreter, &expression) {
            Ok(value) => match value {
                Value::Number(number) => println!("{}", number),
                Value::Function(function) => println!("#<Function:{}>", function.identifier()),
                _ => println!("nil"),
            },

            Err(error) => println!("{:?}", error),
        }
    }
}

fn parse_and_evaluate(interpreter: &mut Interpreter, input: &str) -> Result<Value, Error> {
    let tokens = risp::tokenize(input);
    let expression = risp::parse_node(&mut tokens.into_iter().peekable())?;

    return interpreter.evaluate(&expression);
}
