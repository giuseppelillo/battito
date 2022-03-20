use crate::generator::Harmonic;
use crate::pattern::error::Error;
use crate::pattern::parser::parsed_measure::{ParsedMeasure, Single};
use crate::pattern::parser::primitives::ParsedEvent;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

use super::Expansion;

pub struct HarmonicGenerator {
    pub fundamental: u32,
    pub steps: u32,
}

impl Expansion for HarmonicGenerator {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        let iter = Harmonic::new(self.fundamental);
        let a: Vec<ParsedMeasure> = iter
            .map(|h| {
                ParsedMeasure::Single(Single::Event(ParsedEvent {
                    value: h.to_string(),
                    probability: 100,
                }))
            })
            .take(self.steps as usize)
            .collect();
        Ok(a)
    }

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        let x = preceded(
            tag("harmonic("),
            terminated(separated_list0(tag(","), digit1), char(')')),
        );
        map(x, |a: Vec<&str>| HarmonicGenerator {
            fundamental: a[0].parse().unwrap(),
            steps: a[1].parse().unwrap(),
        })(input)
    }
}
