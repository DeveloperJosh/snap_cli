pub struct Arg<'a> {
    pub name: &'a str,
    pub about: &'a str,
    pub is_flag: bool,
}

impl<'a> Arg<'a> {
    pub fn new(name: &'a str) -> Self {
        Arg {
            name,
            about: "",
            is_flag: false,
        }
    }

    pub fn about(mut self, about: &'a str) -> Self {
        self.about = about;
        self
    }

    pub fn is_flag(mut self, is_flag: bool) -> Self {
        self.is_flag = is_flag;
        self
    }
}
