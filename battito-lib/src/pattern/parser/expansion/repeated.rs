use crate::pattern::error::Error;
use crate::pattern::parser::parsed_measure::ParsedMeasure;
use crate::pattern::parser::repeated;
use nom::IResult;

use super::Expansion;

pub struct Repeated {
    pub(crate) value: ParsedMeasure,
    pub(crate) repetitions: usize,
}

impl Expansion for Repeated {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        Ok(vec![ParsedMeasure::Group(vec![self.value.clone(); self.repetitions])])
    }

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        repeated::parser(input)
    }
}
