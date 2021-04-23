use crate::error::Error;
use crate::parsed_measure::ParsedMeasure;
use nom::combinator::map_res;
use nom::IResult;

pub trait Expansion {
    fn expand(&self) -> Result<ParsedMeasure, Error>;

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;

    fn parse(input: &str) -> IResult<&str, ParsedMeasure>
    where
        Self: Sized,
    {
        map_res(Self::parser, |t| t.expand())(input)
    }
}
