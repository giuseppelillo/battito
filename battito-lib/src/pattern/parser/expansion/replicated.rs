use crate::pattern::error::Error;
use crate::pattern::parser::parsed_measure::ParsedMeasure;
use crate::pattern::parser::replicated;
use nom::IResult;

use super::Expansion;

pub struct Replicated {
    pub value: ParsedMeasure,
    pub replications: usize,
}

impl Expansion for Replicated {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        Ok(vec![self.value.clone(); self.replications])
    }

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        replicated::parser(input)
    }
}
