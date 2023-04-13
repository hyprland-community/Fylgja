// get project info from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

const SYSDEV_HELP: &str = "fylgja is a multi-purpose command-line interface for system administration tasks.

    Usage: fyg [COMMAND] [ARGUMENTS]
    
    Available commands:

        [Project Management]

        > init --name [NAME] --path [PATH] --lang [LANGUAGE] --git [GIT] 
            Initialize a new project with the given parameters
            Example: fyg init --name \"My Project\" --lang \"Rust\" -path\"/home/user/projects/my-project\" --git

        [System Management]

        [Package Management]

";

pub fn help() {
    println!("{}", SYSDEV_HELP);
}

pub fn version() {
    println!("{} {}", NAME, VERSION);
}
