mod commands;
pub use commands::Commands;

mod cli;
mod cli_menus;

use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct CliArguments {
    command: Option<String>,
}

fn main() {
    let args = CliArguments::parse();

    let command;
    match args.command.clone() {
        Some(com) => {
            command = match Commands::from_string(com) {
                Ok(value) => value,
                Err(_) => {
                    panic!("Unknown Command: {:?}", args.command.unwrap().red())
                }
            }
        }
        None => {
            command = match cli::filter_choice_cli(Commands::get_gum_string()) {
                Ok(com) => com,
                Err(val) => {
                    println!("{}", val);
                    std::process::exit(1);
                }
            }
        }
    }
    println!("{}", command.to_string());
    let result = match command {
        Commands::Add => cli_menus::git_add_cli(),
        Commands::Reset => todo!(),
        Commands::Commit => todo!(),
    };
}
