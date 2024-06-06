use crate::{app::AppError, arg::Arg};
use std::collections::HashMap;

/// Represents a command in the command-line application.
pub struct Command<'a> {
    /// The name of the command.
    pub name: &'a str,
    /// A brief description of the command.
    pub about: &'a str,
    /// A list of arguments that the command accepts.
    pub args: Vec<Arg<'a>>,
    /// A HashMap of subcommands for this command.
    pub subcommands: HashMap<&'a str, Command<'a>>,
    /// An optional function to execute when the command is called.
    pub execute: Option<Box<dyn Fn(&Matches) + 'a>>,
}

impl<'a> Command<'a> {
    /// Creates a new instance of `Command`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the command.
    ///
    /// # Returns
    ///
    /// * A new instance of `Command`.
    pub fn new(name: &'a str) -> Self {
        Command {
            name,
            about: "",
            args: Vec::new(),
            subcommands: HashMap::new(),
            execute: None,
        }
    }

    /// Sets the description of the command.
    ///
    /// # Arguments
    ///
    /// * `about` - The description of the command.
    ///
    /// # Returns
    ///
    /// * An instance of `Command` with the description set.
    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    /// Adds an argument to the command.
    ///
    /// # Arguments
    ///
    /// * `arg` - An argument to be added.
    ///
    /// # Returns
    ///
    /// * An instance of `Command` with the argument added.
    pub fn arg(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);
        self
    }

    /// Adds a subcommand to the command.
    ///
    /// # Arguments
    ///
    /// * `subcommand` - A subcommand to be added.
    ///
    /// # Returns
    ///
    /// * An instance of `Command` with the subcommand added.
    pub fn subcommand(mut self, subcommand: Command<'a>) -> Self {
        self.subcommands.insert(subcommand.name, subcommand);
        self
    }

    /// Sets the function to execute when the command is called.
    ///
    /// # Arguments
    ///
    /// * `func` - A function to be executed.
    ///
    /// # Returns
    ///
    /// * An instance of `Command` with the execute function set.
    pub fn execute<F>(mut self, func: F) -> Self
    where
        F: Fn(&Matches) + 'a,
    {
        self.execute = Some(Box::new(func));
        self
    }
}

/// A struct to hold the matches of the parsed command line arguments.
pub struct Matches {
    /// A HashMap to store the parsed arguments.
    pub args: HashMap<String, String>,
}

impl Matches {
    /// Creates a new instance of `Matches`.
    ///
    /// # Returns
    ///
    /// * A new instance of `Matches`.
    pub fn new() -> Self {
        Matches {
            args: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the `Matches`.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    /// * `value` - The value of the argument.
    pub fn insert(&mut self, key: &str, value: String) {
        self.args.insert(key.to_string(), value);
    }

    /// Retrieves the value of a specific argument.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * An `Option` containing the value of the argument if it exists.
    pub fn value_of(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }

    /// Checks if a specific argument is present.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * `true` if the argument is present, `false` otherwise.
    pub fn is_present(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }

    /// Retrieves the value of a specific argument.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * A `Result` containing the value of the argument if it exists, otherwise an `AppError`.
    pub fn get(&self, key: &str) -> Result<&String, AppError> {
        self.args.get(key).ok_or(AppError::MissingValue(key.to_string()))
    }
}
