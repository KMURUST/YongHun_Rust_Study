use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("Enter an expression (or q to quit): ");
        stdout.flush().expect("failed to flush");
        let mut input = String::new();
        stdin.read_line(&mut input).expect("failed to read line");

        if input.trim() == "q" {
            break;
        }

        let tokens: Vec<&str> = input.split_ascii_whitespace().collect();

        if tokens.len() != 3 {
            println!("Invalid expression");
            continue;
        }

        let left: f64 = match tokens[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        let right: f64 = match tokens[2].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                println!("Invalid operator");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}