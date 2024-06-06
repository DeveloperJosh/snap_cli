/// Represents a command-line argument.
use crate::command::Matches;

pub struct Arg<'a> {
    /// The name of the argument.
    pub name: &'a str,
    /// A brief description of the argument.
    pub about: &'a str,
    /// Indicates if the argument is a flag (i.e., does not take a value).
    pub is_flag: bool,
    /// The default value of the argument.
    pub default: Option<&'a str>,
}

impl<'a> Arg<'a> {
    /// Creates a new instance of `Arg`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Returns
    ///
    /// * A new instance of `Arg`.
    pub fn new(name: &'a str) -> Self {
        Arg {
            name,
            about: "",
            is_flag: false,
            default: None,
        }
    }

    /// Sets the description of the argument.
    ///
    /// # Arguments
    ///
    /// * `about` - The description of the argument.
    ///
    /// # Returns
    ///
    /// * An instance of `Arg` with the description set.
    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    /// Sets whether the argument is a flag.
    ///
    /// # Arguments
    ///
    /// * `is_flag` - A boolean indicating if the argument is a flag.
    ///
    /// # Returns
    ///
    /// * An instance of `Arg` with the flag status set.
    pub fn is_flag(mut self, is_flag: bool) -> Self {
        self.is_flag = is_flag;
        self
    }

    /// Sets the default value of the argument.
    ///
    /// # Arguments
    ///
    /// * `default` - The default value of the argument.
    ///
    /// # Returns
    ///
    /// * An instance of `Arg` with the default value set.
    pub fn default(mut self, default: &'a str) -> Self {
        self.default = Some(default);
        self
    }
}

impl Matches {
    /// Get a value of an argument as a string.
    pub fn value_of_str(&self, key: &str, default: &str) -> String {
        self.value_of(key).unwrap_or(&default.to_string()).to_string()
    }

    /// Get a value of an argument as an integer.
    pub fn value_of_int(&self, key: &str, default: i32) -> i32 {
        self.value_of(key)
            .unwrap_or(&default.to_string())
            .parse::<i32>()
            .unwrap_or(default)
    }
}
