use crate::arg::Arg;
use crate::command::{Command, Matches};
use std::collections::HashMap;
use thiserror::Error;

/// Custom error types for the application.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("No arguments provided.")]
    NoArguments,
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    #[error("Missing value for argument: {0}")]
    MissingValue(String),
    #[error("Invalid value for argument: {0}")]
    InvalidValue(String),
    #[error("Unknown argument: {0}")]
    UnknownArgument(String),
    // unexpected argument
    #[error("Unexpected argument: {0}")]
    UnexpectedArgument(String),
}

/// A simple command line argument parser.
pub struct App<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub author: &'a str,
    pub about: &'a str,
    pub args: Vec<Arg<'a>>,
    pub commands: HashMap<&'a str, Command<'a>>,
}

impl<'a> App<'a> {
    pub fn new(name: &'a str) -> Self {
        App {
            name,
            version: "",
            author: "",
            about: "",
            args: Vec::new(),
            commands: HashMap::new(),
        }
    }

    pub fn version(mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }

    pub fn author(mut self, author: &'a str) -> Self {
        self.author = author;
        self
    }

    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    pub fn arg(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);
        self
    }

    pub fn command(mut self, command: Command<'a>) -> Self {
        self.commands.insert(command.name, command);
        self
    }

    pub fn get_matches(&self) -> Result<Matches, AppError> {
        crate::parser::parse(self)
    }

    pub fn print_help(&self) {
        println!("{} {}", self.name, self.version);
        println!("{}", self.about);
        println!("Author: {}", self.author);
        println!("\nUsage:");
        for arg in &self.args {
            if arg.is_positional {
                println!("    {}: {}", arg.name, arg.about);
            } else {
                println!("    --{}: {}", arg.name, arg.about);
            }
        }
        if !self.commands.is_empty() {
            println!("\nCommands:");
            for (name, command) in &self.commands {
                println!("    {}: {}", name, command.about);
                for arg in &command.args {
                    if arg.is_positional {
                        println!("        {}: {}", arg.name, arg.about);
                    } else {
                        println!("        --{}: {}", arg.name, arg.about);
                    }
                }
                if !command.subcommands.is_empty() {
                    println!("        Subcommands:");
                    for (sub_name, sub_command) in &command.subcommands {
                        println!("            {}: {}", sub_name, sub_command.about);
                        for arg in &sub_command.args {
                            if arg.is_positional {
                                println!("                {}: {}", arg.name, arg.about);
                            } else {
                                println!("                --{}: {}", arg.name, arg.about);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn print_command_help(&self, command: &Command) {
        println!("{}", command.about);
        println!("\nUsage:");
        for arg in &command.args {
            if arg.is_positional {
                println!("    {}: {}", arg.name, arg.about);
            } else {
                println!("    --{}: {}", arg.name, arg.about);
            }
        }
        if !command.subcommands.is_empty() {
            println!("\nSubcommands:");
            for (name, subcommand) in &command.subcommands {
                println!("    {}: {}", name, subcommand.about);
                for arg in &subcommand.args {
                    if arg.is_positional {
                        println!("        {}: {}", arg.name, arg.about);
                    } else {
                        println!("        --{}: {}", arg.name, arg.about);
                    }
                }
            }
        }
    }
}
