use risp::{Error, Interpreter, Value};
use std::io::Write;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    filename: Option<String>,
}

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
    let cli = Cli::parse();

    match &cli.filename {
        Some(filename) => {
            let mut interpreter = Interpreter::new();

            match interpreter.evaluate_file(&filename) {
                Ok(_) => (),
                Err(error) => {
                    println!("{:?}", error);
                    std::process::exit(1);
                }
            }
        }

        None => {
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
    }
}

fn parse_and_evaluate(interpreter: &mut Interpreter, input: &str) -> Result<Value, Error> {
    let tokens = risp::tokenize(input);
    let expression = risp::parse_node(&mut tokens.into_iter().peekable())?;

    return interpreter.evaluate(&expression);
}
