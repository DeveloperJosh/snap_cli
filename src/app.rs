use crate::arg::Arg;
use crate::command::{Command, Matches};
use std::collections::HashMap;
use std::process::exit;
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
}

/// A simple command line argument parser.
pub struct App<'a> {
    /// Name of the application.
    name: &'a str,
    /// Version of the application.
    version: &'a str,
    /// Author of the application.
    author: &'a str,
    /// Brief description of the application.
    about: &'a str,
    /// List of arguments for the application.
    args: Vec<Arg<'a>>,
    /// HashMap of commands for the application.
    commands: HashMap<&'a str, Command<'a>>,
}

impl<'a> App<'a> {
    /// Creates a new instance of `App`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the application.
    ///
    /// # Returns
    ///
    /// * A new instance of `App`.
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

    /// Sets the version of the application.
    ///
    /// # Arguments
    ///
    /// * `version` - The version of the application.
    ///
    /// # Returns
    ///
    /// * An instance of `App` with the version set.
    pub fn version(mut self, version: &'a str) -> Self {
        self.version = version;
        self
    }

    /// Sets the author of the application.
    ///
    /// # Arguments
    ///
    /// * `author` - The author of the application.
    ///
    /// # Returns
    ///
    /// * An instance of `App` with the author set.
    pub fn author(mut self, author: &'a str) -> Self {
        self.author = author;
        self
    }

    /// Sets the description of the application.
    ///
    /// # Arguments
    ///
    /// * `about` - The description of the application.
    ///
    /// # Returns
    ///
    /// * An instance of `App` with the description set.
    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    /// Adds an argument to the application.
    ///
    /// # Arguments
    ///
    /// * `arg` - An argument to be added.
    ///
    /// # Returns
    ///
    /// * An instance of `App` with the argument added.
    pub fn arg(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);
        self
    }

    /// Adds a command to the application.
    ///
    /// # Arguments
    ///
    /// * `command` - A command to be added.
    ///
    /// # Returns
    ///
    /// * An instance of `App` with the command added.
    pub fn command(mut self, command: Command<'a>) -> Self {
        self.commands.insert(command.name, command);
        self
    }

    /// Parses the command line arguments and returns the matches.
    ///
    /// # Returns
    ///
    /// * An instance of `Matches` containing the parsed arguments or an `AppError`.
    pub fn get_matches(&self) -> Result<Matches, AppError> {
        let args: Vec<String> = std::env::args().collect();

        // Check if no arguments are provided
        if args.len() == 1 {
            self.print_help();
            return Err(AppError::NoArguments);
        }

        // Check for help flag first
        if args.iter().any(|arg| arg == "--help" || arg == "-h") {
            self.print_help();
            exit(0);
        }

        let mut matches = Matches::new();

        // Parse global arguments
        self.parse_args(&mut matches, &self.args, &args)?;

        // Check for command execution
        if args.len() > 1 {
            if let Some(command) = self.commands.get(args[1].as_str()) {
                let mut command_matches = self.parse_command_args(command, &args[2..])?;
                for (k, v) in matches.args.iter() {
                    command_matches.insert(k, v.clone());
                }
                if let Some(execute) = &command.execute {
                    execute(&command_matches);
                    exit(0);
                } else {
                    self.handle_subcommand(command, &args[2..]);
                }
            } else {
                return Err(AppError::UnknownCommand(args[1].clone()));
            }
        }

        Ok(matches)
    }

    /// Parses the command line arguments.
    ///
    /// # Arguments
    ///
    /// * `matches` - The `Matches` instance to store the parsed arguments.
    /// * `arg_definitions` - The list of argument definitions.
    /// * `args` - The list of command line arguments.
    fn parse_args(&self, matches: &mut Matches, arg_definitions: &[Arg], args: &[String]) -> Result<(), AppError> {
        let mut i = 0;
        while i < args.len() {
            if args[i].starts_with("--") {
                let arg_name = &args[i][2..];
                if let Some(arg_def) = arg_definitions.iter().find(|a| a.name == arg_name) {
                    if arg_def.is_flag {
                        matches.insert(arg_name, "true".to_string());
                    } else if i + 1 < args.len() {
                        matches.insert(arg_name, args[i + 1].clone());
                        i += 1;
                    } else if let Some(default) = arg_def.default {
                        matches.insert(arg_name, default.to_string());
                    } else {
                        return Err(AppError::MissingValue(arg_name.to_string()));
                    }
                } else {
                    // Handle custom arguments
                    if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                        matches.insert(arg_name, args[i + 1].clone());
                        i += 1;
                    } else {
                        matches.insert(arg_name, "true".to_string());
                    }
                }
            }
            i += 1;
        }
        Ok(())
    }

    /// Parses the command line arguments for a specific command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command definition.
    /// * `args` - The list of command line arguments.
    ///
    /// # Returns
    ///
    /// * An instance of `Matches` containing the parsed arguments.
    fn parse_command_args(&self, command: &Command, args: &[String]) -> Result<Matches, AppError> {
        let mut matches = Matches::new();
        self.parse_args(&mut matches, &command.args, args)?;

        Ok(matches)
    }

    /// Handles the subcommands of a command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command definition.
    /// * `args` - The list of command line arguments.
    fn handle_subcommand(&self, command: &Command, args: &[String]) {
        if !args.is_empty() {
            if let Some(subcommand) = command.subcommands.get(args[0].as_str()) {
                let subcommand_matches = self.parse_command_args(subcommand, &args[1..]).unwrap();
                if let Some(execute) = &subcommand.execute {
                    execute(&subcommand_matches);
                } else {
                    self.print_command_help(subcommand);
                }
                exit(0);
            }
        }
        self.print_command_help(command);
    }

    /// Prints the help information for the application.
    fn print_help(&self) {
        println!("{} {}", self.name, self.version);
        println!("{}", self.about);
        println!("Author: {}", self.author);
        println!("\nUsage:");
        for arg in &self.args {
            println!("    {}: {}", arg.name, arg.about);
        }
        if !self.commands.is_empty() {
            println!("\nCommands:");
            for (name, command) in &self.commands {
                println!("    {}: {}", name, command.about);
                for (sub_name, sub_command) in &command.subcommands {
                    println!("        {}: {}", sub_name, sub_command.about);
                }
            }
        }
    }

    /// Prints the help information for a specific command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command definition.
    fn print_command_help(&self, command: &Command) {
        println!("{}", command.about);
        println!("\nUsage:");
        for arg in &command.args {
            println!("    {}: {}", arg.name, arg.about);
        }
        if !command.subcommands.is_empty() {
            println!("\nSubcommands:");
            for (name, subcommand) in &command.subcommands {
                println!("    {}: {}", name, subcommand.about);
                for arg in &subcommand.args {
                    println!("        {}: {}", arg.name, arg.about);
                }
            }
        }
    }
}
