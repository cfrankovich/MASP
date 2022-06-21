use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::str::SplitWhitespace;

enum CommandTypes {
    OK,
    ParenthesisMismatch,
    NotEnclosed,
    InvalidCommand,
    InvalidArgs,
}

fn get_command_info(command: &String) -> (String, usize) {
    let bytes = command.as_bytes();
    let mut word: String = String::new();
    for i in 1..command.len()-1 {
        let c: char = bytes[i] as char;
        if c == ' ' || c == ')' {
            break;
        }
        word.push_str(&c.to_string());
    }
    let split: SplitWhitespace = command.split_whitespace();
    return (String::from(word), split.count());
}

fn is_valid_command(command: &String) -> CommandTypes {
    /* check syntax */
    let mut open_par_count: u32 = 0;
    let mut close_par_count: u32 = 0;
    for c in command.chars() {
        if c == '(' { open_par_count += 1; }
        else if c == ')' { close_par_count += 1; }
    }
    if open_par_count != close_par_count {
        return CommandTypes::ParenthesisMismatch;
    }

    let first_char: char = command.as_bytes()[0] as char;
    let last_char: char = command.as_bytes()[command.len()-2] as char;
    if first_char != '(' || last_char != ')' {
        return CommandTypes::NotEnclosed;
    }

    /* check list of commands */ 
    let command_info: (String, usize) = get_command_info(&command); 
    let command_list: HashMap<&str,usize> = HashMap::from([
        ("exit", 0),
        ("ping", 0),
    ]);
    for com in command_list {
        if com.0.eq(&command_info.0) {
            if com.1 == command_info.1-1 {
                return CommandTypes::OK;
            } else {
                return CommandTypes::InvalidArgs;
            }
        }
    }

    return CommandTypes::InvalidCommand;
}

fn execute_command(_command: String) {

}

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("Welcome to MASP v{}", VERSION);

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut fullcommand: String = String::new();
        io::stdin()
            .read_line(&mut fullcommand)
            .expect("[Error] Failed to read line.");
        if fullcommand.eq("\n") {
            continue;
        }

        match is_valid_command(&fullcommand) {
            CommandTypes::OK => execute_command(fullcommand),
            CommandTypes::ParenthesisMismatch => {
                eprintln!("[Error] Parenthesis mismatch.");
            },
            CommandTypes::NotEnclosed => {
                eprintln!("[Error] Invalid syntax.");
            },
            CommandTypes::InvalidCommand => {
                eprintln!("[Error] Invalid command.");
            },
            CommandTypes::InvalidArgs => {
                eprintln!("[Error] Invalid number of arguments.");
            },
        }
    }

}
