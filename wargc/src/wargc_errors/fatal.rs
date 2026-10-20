use colored::Colorize;

pub fn throw_fatal(cause: &str) -> ! {
    eprintln!("{}: Compiler error: {cause}", "error".bold().red());
    std::process::exit(3);
}