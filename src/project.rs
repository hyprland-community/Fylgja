use colored::*;
use std::env;
use std::fs::create_dir;


pub struct project {
    pub language: String, // programming language
    pub name: String,     // project name
    pub path: String,     // project path
    pub git: bool,        // should the project be initialized with git?
}

// function to create a new project
pub fn new_project() {
    // get the arguments passed to the program
    let args: Vec<String> = env::args().collect();

    let mut language = String::new();
    let mut name = String::new();
    let mut path = String::new();
    let mut git = false;

    // if the program is run without any arguments, display an error message
    if args.len() == 1 {
        println!("Error: No arguments passed. Run 'sysdev --help' for more information.");
    } else {
        if args.len() > 1 {
            let command = &args[1];
            let name = &args[2];
            let path = &args[4];
            let language = &args[3];
            let git = &args[5] || false;

            match &command[..] {
                new => {
                    println!("Creating new project with given parameters...");
                    println!("Project name: {}", name);
                    println!("Project path: {}", path);
                    println!("Project language: {}", language);
                    println!("Project git: {}", git);
                    println!("Is this correct? [Y/n]");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).expect("Failed to read line");
                    match input.trim() {
                        "Y" | "y" => {
                            println!("Creating project...");
                            create_dir(path);

                        }
                        "N" | "n" => {
                            println!("Aborting...");
                            // TODO: make this start the process over again
                        }
                        _ => {
                            println!("Invalid input. Aborting...");
                        }
                    }
                }
            }
        }
}