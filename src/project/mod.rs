use std::env;
use std::fs;
use std::process::Command;

pub struct Project {
    pub language: String, // programming language
    pub name: String,     // project name
    pub path: String,     // project path
    pub git: bool,        // should the project be initialized with git?
}

impl Project {
    // function to create a new project using the arguments passed to the program
    pub fn new_project(args: Vec<String>) {
        // get the arguments passed to the program
        //let args: Vec<String> = env::args().collect();

        // if args are greater than 1, and first arg is "init" then we have a valid command
        if args.len() > 1 && &args[1] == "init" {
            let mut language = String::new();
            let mut name = String::new();
            let mut path = String::new();
            let mut git = false;

            // iterate over the arguments
            for (i, arg) in args.iter().enumerate() {
                match &arg[..] {
                    "-l" | "--language" => {
                        language = args[i + 1].to_string();
                    }
                    "-n" | "--name" => {
                        name = args[i + 1].to_string();
                    }
                    "-p" | "--path" => {
                        path = args[i + 1].to_string();
                    }
                    "-g" | "--git" => {
                        git = true;
                    }
                    _ => {
                        // no arguments passed, command cannot proceed without arguments
                        println!("You need to supply arguments to the command. Use \"sysdev help init\" for more information.");
                    }
                }
            }

            // use the arguments to create a new project
            let project = Project {
                language,
                name,
                path,
                git,
            };

            fn create_project(project: Project) {
                // create the project directory
                let project_path = format!("{}/{}", project.path, project.name);
                let project_path = project_path.replace("\"", "");
                fs::create_dir_all(project_path.clone())
                    .expect("Failed to create project directory");

                // check the language argument and create the appropriate files based on the language
                match &project.language[..] {
                    "rust" => {
                        // initialise cargo in project directory with --vsc none if git is false
                        if project.git == false {
                            Command::new("cargo")
                                .arg("init")
                                .arg("--vcs")
                                .arg("none")
                                .current_dir(project_path)
                                .output()
                                .expect("Failed to initialize cargo");
                        } else {
                            Command::new("cargo")
                                .arg("init")
                                .current_dir(project_path)
                                .output()
                                .expect("Failed to initialize cargo");
                        }
                    }

                    "go" => {
                        // initialise go mod in project directory
                        Command::new("go")
                            .arg("mod")
                            .arg("init")
                            .current_dir(project_path)
                            .output()
                            .expect("Failed to initialize go mod");
                    }

                    "python" => {
                        // create a main.py file in the project directory
                        let main_file = format!("{}/main.py", project_path);
                        fs::File::create(main_file).expect("Failed to create main.py file");
                    }

                    "c" => {
                        // create a main.c file in the project directory
                        let main_file = format!("{}/main.c", project_path);
                        fs::File::create(main_file).expect("Failed to create main.c file");
                    }

                    _ => {
                        // if no arguments are passed, return nothing and exit
                        if project.language == "" {
                            return;
                        } else {
                            println!(
                                "Invalid language. Use \"sysdev help init\" for more information."
                            );
                        }
                    }
                }
            }

            create_project(project);
        } else {
            println!("Invalid command. Use \"sysdev help\" for more information.");
        }
    }
}
