/// Represents a command-line argument.
use crate::command::Matches;

pub struct Arg<'a> {
    /// The name of the argument.
    pub name: &'a str,
    /// A short version of the argument name.
    pub short: Option<&'a str>,
    /// A brief description of the argument.
    pub about: &'a str,
    /// Indicates if the argument is a flag (i.e., does not take a value).
    pub is_flag: bool,
    /// Indicates if the argument is positional.
    pub is_positional: bool,
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
            short: None,
            about: "",
            is_flag: false,
            is_positional: false,
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

    /// Sets a short version of the argument name.
    ///
    /// # Arguments
    ///
    /// * `short` - The short version of the argument name.
    ///
    /// # Returns
    ///
    /// * An instance of `Arg` with the short version set.
    pub fn short(mut self, short: &'a str) -> Self {
        self.short = Some(short);
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

    /// Sets whether the argument is positional.
    ///
    /// # Arguments
    ///
    /// * `is_positional` - A boolean indicating if the argument is positional.
    ///
    /// # Returns
    ///
    /// * An instance of `Arg` with the positional status set.
    pub fn is_positional(mut self, is_positional: bool) -> Self {
        self.is_positional = is_positional;
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