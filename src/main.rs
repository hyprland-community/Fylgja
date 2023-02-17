use std::env;

// my modules
use project::{new_project};
//use lib::{help};

// version number - TODO: automate this later
const VERSION: &str = "0.0.1";

// program name - for if I ever want to change it
const NAME: &str = "sysdev";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {

        // display the welcome banner enclosed in a box - TODO: make this look better because it looks ugly
        println!("┌──────────────────────────────────────────────┐");
        println!("│    Welcome to {} {}                   │", NAME, VERSION);
        println!("│     A multi-purpose command-line interface   │");
        println!("│     for system administration tasks.         │");
        println!("└──────────────────────────────────────────────┘");

    } else {
        if args.len() > 1 {
            let command = &args[1];
            match &command[..] {
                // "help" | "--help" | "-h" | _ => help(),
                "version" | "--version" | "-v" => println!("{} {}", NAME, VERSION),
            }
        }
    }
}