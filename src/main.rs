use std::io;
use std::io::Write;

enum CommandTypes {
    OK,
    ParenthesisMismatch,
    NotEnclosed,
    InvalidCommand,
}

fn get_command_word(command: &String) -> String {
    let bytes = command.as_bytes();
    let mut word: String = String::new();
    for i in 1..command.len()-1 {
        let c: char = bytes[i] as char;
        if c == ' ' || c == ')' {
            break;
        }
        word.push_str(&c.to_string());
    }
    return String::from(word);
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
    let command_word: String = get_command_word(&command); 
    const NUM_COMMANDS: usize = 2;
    const COMMAND_LIST: [&str; NUM_COMMANDS] = [
        "exit",
        "ping",
    ];
    for com in COMMAND_LIST {
        if com.eq(&command_word) {
            return CommandTypes::OK;
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
            .expect("Failed to read line.");

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
        }
    }

}
