use crate::generator::Binary;
use crate::pattern::error::Error;
use crate::pattern::parser::parsed_measure::ParsedMeasure;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

use super::Expansion;

impl Expansion for Binary {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        let a = self
            .generate()
            .iter()
            .map(|h| {
                if h > &0 {
                    ParsedMeasure::event("0")
                } else {
                    ParsedMeasure::event("~")
                }
            })
            .collect();
        Ok(a)
    }

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let x = preceded(tag("binary("), terminated(separated_list0(tag(","), digit1), char(')')));
        map(x, |a: Vec<&str>| Binary {
            number: a[0].parse().unwrap(),
            length: a[1].parse().unwrap(),
        })(input)
    }
}
