use crate::util::number::Number;

pub enum ParameterValue {
    Text(String),
    Numeric(Number),
}

pub type ParameterNameAndValue<'a> = (&'a str, ParameterValue);
