// get project info from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

// help message with ansii color codes
const HELP_TEXT: &str = "\x1b[1;31mfylgja is a multi-purpose command-line interface for system administration tasks.\x1b[0m

    \x1b[34mUsage: fyg [COMMAND] [ARGUMENTS]\x1b[0m
    
    \x1b[1;31mAvailable commands:\x1b[0m

        \x1b[32m[Project Management]\x1b[0m

        > \x1b[35minit\x1b[0m \x1b[31m--name\x1b[0m \x1b[32m[NAME]\x1b[0m \x1b[33m--path\x1b[0m \x1b[32m[PATH]\x1b[0m \x1b[34m--lang\x1b[0m \x1b[32m[LANGUAGE]\x1b[0m \x1b[35m--git\x1b[0m \x1b[32m[GIT]\x1b[0m 
            Initialize a new project with the given parameters
            Example: fyg init --name \"My Project\" --lang \"Rust\" --path\"/home/user/projects/my-project\" --git

        \x1b[32m[System Management]\x1b[0m

        \x1b[32m[Package Management]\x1b[0m";

pub fn help() {
    println!("{}", HELP_TEXT);
}

pub fn version() {
    println!("{} {}", NAME, VERSION);
}
