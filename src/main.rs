use std::env;

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
                println!("┌──────────────────────────────────────────────┐");
                println!(
                    "│    Welcome to {} {}                   │",
                    fylgja::NAME,
                    fylgja::VERSION
                );
                println!("│     A multi-purpose command-line interface   │");
                println!("│     for system administration tasks.         │");
                println!("└──────────────────────────────────────────────┘");
            }
        }
    }
}
