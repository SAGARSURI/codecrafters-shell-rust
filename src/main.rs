#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::{exit, ExitCode};

fn main() -> ExitCode {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let input = read_line();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts.get(0);
        let args = parts.get(1..);
        match command {
            Some(&"exit") => {
                if let Some(arg_list) = args {
                    exit(arg_list.get(0).unwrap().parse::<i32>().unwrap());
                }
            },
            _ => println!("{}: command not found", input)
            
        }
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    input.trim().to_string()
}
