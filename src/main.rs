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
            Some(&cmd) if cmd == Commands::Exit.to_string() => {
                if !args.is_empty() {
                    exit(args[0].parse::<i32>().unwrap_or(0));
                }
            }
            Some(&cmd) if cmd == Commands::Echo.to_string() => println!("{}", args.join(" ")),
            Some(&cmd) if cmd == Commands::Type.to_string() => {
                if let Some(&first_arg) = args.first() {
                    match first_arg {
                        arg if arg == Commands::Echo.to_string()
                            || arg == Commands::Exit.to_string()
                            || arg == Commands::Type.to_string() =>
                        {
                            println!("{} is a shell builtin", arg);
                        }
                        _ => println!("{}: not found", first_arg),
                    }
                }
            }
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

enum Commands {
    Echo,
    Exit,
    Type,
}

impl Commands {
    fn to_string(&self) -> &str {
        match self {
            Commands::Echo => "echo",
            Commands::Exit => "exit",
            Commands::Type => "type",
        }
    }
}
