use crate::parameter::ParameterNameAndValue;

pub struct Command<'a> {
    /// The Command name (without parameter substitution)
    name: Option<&'a str>,

    /// The command that should be executed (without parameter substitution)
    expression: &'a str,

    parameters: Vec<ParameterNameAndValue<'a>>,
}

impl<'a> Command<'a> {
    pub fn new(name: Option<&'a str>, expression: &'a str) -> Self {
        Command {
            name,
            expression,
            parameters: Vec::new(),
        }
    }
}

pub struct Commands<'a>(Vec<Command<'a>>);

impl<'a> Commands<'a> {
    pub fn iter(&self) -> impl Iterator<Item = &Command<'a>> {
        self.0.iter()
    }
}
