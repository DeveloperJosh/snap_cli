/// Represents a command-line argument.
pub struct Arg<'a> {
    /// The name of the argument.
    pub name: &'a str,
    /// A brief description of the argument.
    pub about: &'a str,
    /// Indicates if the argument is a flag (i.e., does not take a value).
    pub is_flag: bool,
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
}
