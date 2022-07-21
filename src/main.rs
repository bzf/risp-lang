use std::io::Write;

use risp::Value;

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

    loop {
        let expression = prompt("> ");

        if expression.is_empty() {
            continue;
        }

        if let Some(value) = risp::parse_and_evaluate(&expression) {
            match value {
                Value::Number(number) => println!("{}", number),
                _ => println!("nil"),
            }
        } else {
            println!("That doesn't look like anything to me.");
        }
    }
}
