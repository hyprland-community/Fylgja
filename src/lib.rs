use colored::Colorize;
// get project info from Cargo.toml
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");

pub fn help() {

    println!("{}\n\n    {} {} {} {}\n\n    {}\n\n       {}\n\n        {} {} {} {} {} {} {} {} {}\n\n           {}\n\n       {}\n\n       {}",
             "fylgja is a multi-purpose command-line interface for system administration tasks.".bold().red(),
             "Usage:", "fyg".blue(), "[COMMAND]".purple(), "[ARGUMENTS]".bright_cyan(),
             "Available commands:".bright_red(),
             "[Project Management]".green(),
             "> init".purple(), "--name".red(), "[NAME]".green(), "--path".yellow(), "[PATH]".green(), "--lang".blue(), "[LANGUAGE]".green(), "--git".magenta(), "[GIT]".green(),
             "Initialize a new project with the given parameters
           Example: fyg init --name \"My Project\" --lang \"Rust\" --path\"/home/user/projects/my-project\" --git".bright_red(),
           "[System Management]".green(),
           "[Package Management]".green()

             )
}

pub fn version() {
    println!("{} {}", NAME, VERSION);
}
