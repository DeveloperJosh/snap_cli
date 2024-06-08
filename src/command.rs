use crate::{app::AppError, arg::Arg};
use std::collections::HashMap;

/// Represents a command in the command-line application.
pub struct Command<'a> {
    pub name: &'a str,
    pub about: &'a str,
    pub args: Vec<Arg<'a>>,
    pub subcommands: HashMap<&'a str, Command<'a>>,
    pub execute: Option<Box<dyn Fn(&Matches) + 'a>>,
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str) -> Self {
        Command {
            name,
            about: "",
            args: Vec::new(),
            subcommands: HashMap::new(),
            execute: None,
        }
    }

    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    pub fn arg(mut self, arg: Arg<'a>) -> Self {
        self.args.push(arg);
        self
    }

    pub fn subcommand(mut self, subcommand: Command<'a>) -> Self {
        self.subcommands.insert(subcommand.name, subcommand);
        self
    }

    pub fn execute<F>(mut self, func: F) -> Self
    where
        F: Fn(&Matches) + 'a,
    {
        self.execute = Some(Box::new(func));
        self
    }

    /// Executes the command if it has an associated function.
    ///
    /// # Arguments
    ///
    /// * `matches` - The `Matches` instance containing parsed arguments.
    ///
    /// # Returns
    ///
    /// * A result indicating success or an error.
    pub fn run(&self, matches: &Matches) -> Result<(), AppError> {
        if let Some(execute_fn) = &self.execute {
            execute_fn(matches);
            Ok(())
        } else {
            Err(AppError::UnknownCommand(self.name.to_string()))
        }
    }
}

/// Represents the parsed command-line arguments.
pub struct Matches {
    pub args: HashMap<String, String>,
}

impl Matches {
    pub fn new() -> Self {
        Matches {
            args: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: &str, value: String) {
        self.args.insert(key.to_string(), value);
    }

    pub fn value_of(&self, key: &str) -> Option<&String> {
        self.args.get(key)
    }

    pub fn is_present(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Result<&String, AppError> {
        self.args.get(key).ok_or(AppError::MissingValue(key.to_string()))
    }

    /// Gets the value of an argument as a string.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * The value as a string or an error if not found.
    pub fn value_of_str(&self, key: &str) -> Result<String, AppError> {
        self.get(key).map(|v| v.to_string())
    }

    /// Gets the value of an argument as an integer.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * The value as an integer or an error if not found or not a valid integer.
    pub fn value_of_int(&self, key: &str) -> Result<i32, AppError> {
        self.get(key)
            .and_then(|v| v.parse::<i32>().map_err(|_| AppError::InvalidValue(key.to_string())))
    }

    /// Checks if an argument is present.
    ///
    /// # Arguments
    ///
    /// * `key` - The key of the argument.
    ///
    /// # Returns
    ///
    /// * `true` if the argument is present, `false` otherwise.
    pub fn contains(&self, key: &str) -> bool {
        self.args.contains_key(key)
    }
}
