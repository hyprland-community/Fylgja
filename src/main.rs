use std::env;
use colored::Colorize;
// get the help function from lib.rs
use fylgja::{help, version};

pub mod project;

// get new_project from project/mod.rs Project impl
use project::Project;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "version" | "--version" | "-v" => version(),
            "help" | "--help" | "-h" => help(),
            "init" => Project::new_project(args),

            _ => {
                help();   
            }
        }
    } else {
        println!("{}","┌──────────────────────────────────────────────┐".blue());
        println!("{}{} {}{}",
                 "│             Welcome to ".blue(),
                 fylgja::NAME.to_string().purple(), 
                 fylgja::VERSION.bright_red(),
                 "          │".blue(),


                 );
        println!("{}","│     A multi-purpose command-line interface   │".blue());
        println!("{}","│         for system administration tasks.     │".blue());
        println!("{}","└──────────────────────────────────────────────┘".blue());

    }
}
