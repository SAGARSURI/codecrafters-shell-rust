#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let input = read_line();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts.get(0);
        let args = parts.get(1..).unwrap_or_default();
        match command {
            Some(&"exit") => {
                if !args.is_empty() {
                    exit(args[0].parse::<i32>().unwrap_or(0));
                }
            }
            Some(&"echo") => println!("{}", args.join(" ")),
            _ => println!("{}: command not found", input),
        }
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input.trim().to_string()
}
