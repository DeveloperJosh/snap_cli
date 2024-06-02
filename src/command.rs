use crate::arg::Arg;
use crate::app::Matches;
use std::collections::HashMap;

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
}
