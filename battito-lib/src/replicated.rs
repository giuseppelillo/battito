use crate::error::Error;
use crate::expansion::Expansion;
use crate::parsed_measure::ParsedMeasure;
use crate::parser::replicated;
use nom::IResult;

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
