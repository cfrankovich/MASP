/*
 * This file contains all functions regarding the standard library in the MASP language. These
 * functions are included automatically. 
 *
 * For additional documentation please reference: https://github.com/cfrankovich/MASP
 *
 * Once the standard library is in a state where documentation is able to be created, it will
 * be in the wiki.
*/

use CommandTypes;
use check_command_and_args;

/*
 * Info: exits the program
 * Args (none): 
*/
pub fn exit() {
    std::process::exit(0);
}

/*
 * Info: adds numbers together
 * Args (inf): the numbers to add 
*/
pub fn add(command: String) -> f32 {
    let mut vec: Vec<f32> = vec![];
    let split: Vec<&str> = command.split(|c| c == ' ' || c == ')' || c == '(').collect(); 
    for itr in 2..split.len()-1 {
        vec.push(split[itr].parse::<f32>().unwrap());
    }
    let mut sum: f32 = 0.0;
    for number in vec.iter() {
        sum += number;
    }
    println!("{}", sum);
    return sum;
}

/*
 * Info: subtracts numbers together
 * Args (inf): the numbers to subtract 
*/
pub fn subtract(command: String) -> f32{
    let mut vec: Vec<f32> = vec![];
    let split: Vec<&str> = command.split(|c| c == ' ' || c == ')' || c == '(').collect(); 
    for itr in 2..split.len()-1 {
        vec.push(split[itr].parse::<f32>().unwrap());
    }
    let mut dif: f32 = vec[0] * 2.0;
    for number in vec.iter() {
        dif -= number;
    }
    println!("{}", dif);
    return dif;
} 

/*
 * Info: multiplies numbers 
 * Args (inf): the numbers to multiply 
*/
pub fn multiply(command: String) -> f32 {
    let mut vec: Vec<f32> = vec![];
    let split: Vec<&str> = command.split(|c| c == ' ' || c == ')' || c == '(').collect(); 
    for itr in 2..split.len()-1 {
        vec.push(split[itr].parse::<f32>().unwrap());
    }
    let mut product: f32 = 1.0;
    for number in vec.iter() {
        product *= number;
    }
    println!("{}", product);
    return product;
}

/*
 * Info: divides numbers 
 * Args (2): first number is being divided by the next 
*/
pub fn divide(command: String) -> f32 {
    let mut vec: Vec<f32> = vec![];
    let split: Vec<&str> = command.split(|c| c == ' ' || c == ')' || c == '(').collect(); 
    for itr in 2..split.len()-1 {
        vec.push(split[itr].parse::<f32>().unwrap());
    }
    let mut quotient: f32 = vec[0];
    vec.remove(0);
    for number in vec.iter() {
        quotient /= number;
    }
    println!("{}", quotient);
    return quotient;
}

/*
 * Info: gets the remainder after dividing 
 * Args (2): first number is being divided by the next 
*/
pub fn modulo(command: String) -> f32 {
    let split: Vec<&str> = command.split(|c| c == ' ' || c == ')' || c == '(').collect(); 
    let num1: f32 = split[2].parse::<f32>().unwrap();
    let num2: f32 = split[3].parse::<f32>().unwrap();

    println!("{}", num1 % num2);
    return num1 % num2;
}

/*
 * Info: displays help for command specified
 * Args (1): the command to display help for
*/
pub fn help(arg1: &str) {
    let cmdtype: CommandTypes = check_command_and_args(&(String::from("(") + &String::from(arg1) + &String::from(")")));
    match cmdtype {
        CommandTypes::InvalidCommand => {
            eprintln!("[Help Error] Invalid command \"{}\"", arg1);
            return;
        }
        _ => (), 
    }

    match arg1 {
        "exit" => {
            println!("# USAGE:");
            println!("#    (exit)");
            println!("# ARGS:");
            println!("#    None.");
            println!("# DESCRIPTION:");
            println!("#     Exits the program safely."); 
        },
        "+" => {
            println!("# USAGE:");
            println!("#    (+ arg1 arg2...)");
            println!("# ARGS:");
            println!("#    Numbers to add together.");
            println!("# DESCRIPTION:");
            println!("#     Returns the result of the arguments added together."); 
        },
        "-" => {
            println!("# USAGE:");
            println!("#    (- arg1 arg2...)");
            println!("# ARGS:");
            println!("#    Numbers to subtract from the first argument.");
            println!("# DESCRIPTION:");
            println!("#     Returns the result of the arguments subtracted from the first."); 
        },
        "*" => {
            println!("# USAGE:");
            println!("#    (* arg1 arg2...)");
            println!("# ARGS:");
            println!("#    Numbers to multiply together.");
            println!("# DESCRIPTION:");
            println!("#     Returns the result of the arguments multiplied together."); 
        },
        "/" => {
            println!("# USAGE:");
            println!("#    (/ arg1 arg2...)");
            println!("# ARGS:");
            println!("#    Numbers to divide from eachother.");
            println!("# DESCRIPTION:");
            println!("#     Returns the result of the arguments divided one after another."); 
        },
        "%" => {
            println!("# USAGE:");
            println!("#    (% arg1 arg2)");
            println!("# ARGS:");
            println!("#    First argument is divided by the second.");
            println!("# DESCRIPTION:");
            println!("#     Returns the remainder of the arguments after divided."); 
            println!("#     For example, (% 5 2) will return 1."); 
        },
        "help" => {
            println!("# USAGE:");
            println!("#    (help arg1)");
            println!("# ARG:");
            println!("#    The function that needs clarification.");
            println!("# DESCRIPTION:");
            println!("#     Prints this menu for the function given."); 
            println!("#     This will only work for functions that are built in."); 
        },
        _ => (),
    }
}

/*
pub fn print() {}
pub fn new() {}
pub fn set() {}
pub fn include() {}
pub fn log() {}
pub fn r#macro() {}
*/

