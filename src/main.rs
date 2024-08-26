#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, process::exit};

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
                        _ => match get_path(first_arg) {
                            Some(path) => {
                                println!("{} is {}", first_arg, path);
                            }
                            _ => println!("{}: not found", first_arg),
                        },
                    }
                }
            }
            Some(&full_path) if get_path(command.unwrap()).is_some() => {
                let output = execute_program(full_path, args);
                println!("{}", output.unwrap());
            }
            _ => println!("{}: command not found", input),
        }
    }
}

fn get_path(cmd: &str) -> Option<String> {
    let sys_path_result = env::var("PATH");
    match sys_path_result {
        Ok(sys_path) => {
            let paths: Vec<&str> = sys_path.split(':').collect();
            for path in paths {
                let full_path = format!("{}/{}", path, cmd);
                if std::fs::metadata(&full_path).is_ok() {
                    return Some(full_path);
                }
            }
        }
        Err(_) => return None,
    }
    return None;
}

fn execute_program(path: &str, args: &[&str]) -> Option<String> {
    let output = std::process::Command::new(path).args(args).output();
    if output.is_ok() {
        return Some(String::from_utf8_lossy(&output.unwrap().stdout).trim().to_string());
    }
    return None;
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
