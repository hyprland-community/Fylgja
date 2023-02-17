use std::env;

// help function
const SYSDEV_HELP: &str = "Usage: sysdev [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: sysdev init -n \"My Project\" -l \"Rust\" -p \"/home/user/projects/my-project\" -g true

Available commands:
    - init -n [NAME] -p [PATH] -l [LANGUAGE] -g [GIT] 
        Initialize a new project with the given parameters
        Example: sysdev init -n \"My Project\" -l \"Rust\" -p \"/home/user/projects/my-project\" -g true

";
pub fn help() {
    println!("{}", SYSDEV_HELP);
}