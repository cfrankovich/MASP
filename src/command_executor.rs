/*
 * All command executions are handled here after being validated.
 *
 * This function is stored in this module to declutter main.rs
*/

use INCLUDED_LIBS;
use get_command_info;
use masp_stdlib;
use SplitWhitespace;

pub fn execute_command(command: String) {
    /* any better way to do this? actually curious */
    let command_info: (String, usize) = get_command_info(&command); 

    if INCLUDED_LIBS.stnd {
        if &command_info.0 == "exit" {
            masp_stdlib::exit();
            return;
        }
        else if &command_info.0 == "+" {
            masp_stdlib::add(command);
            return;
        }
        else if &command_info.0 == "-" {
            masp_stdlib::subtract(command);
            return;
        }
        else if &command_info.0 == "*" {
            masp_stdlib::multiply(command);
            return;
        }
        else if &command_info.0 == "/" {
            masp_stdlib::divide(command);
            return;
        }
        else if &command_info.0 == "%" {
            masp_stdlib::modulo(command);
            return;
        }
        else if &command_info.0 == "help" {
            let subcommand: &str = &command[1..command.len()-3];
            let mut split: SplitWhitespace = subcommand.split_whitespace();  
            split.next();
            match split.next() {
                Some(arg) => masp_stdlib::help(&arg),
                None => return,
            }
            return;
        }
    }

}


