use crate::error::Error;
use crate::expansion::Expansion;
use crate::parsed_measure::ParsedMeasure;
use crate::parser::{parser_parsed_measure, inner_parser_group, parser_single};
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, map};
use nom::sequence::{preceded, tuple};
use nom::{IResult, ParseTo};
use nom::branch::alt;

pub struct Repeated {
    value: ParsedMeasure,
    repetitions: usize,
}

impl Expansion for Repeated {
    fn expand(&self) -> Result<ParsedMeasure, Error> {
        Ok(ParsedMeasure::Group(vec![self.value.clone(); self.repetitions]))
    }

    fn parser(input: &str) -> IResult<&str, Self>
    where
        Self: Sized,
    {
        parser(input)
    }
}

fn parser(input: &str) -> IResult<&str, Repeated> {
    map_res(
        tuple((inner_parser, preceded(char('*'), digit1))),
        |(pm, digit)| -> Result<Repeated, Error> {
            let repetitions: Result<u32, Error> = digit.parse().map_err(Error::from);
            Ok(Repeated {
                value: pm,
                repetitions: digit.parse().unwrap()
            })
        },
    )(input)
}

fn inner_parser(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((inner_parser_group, parser_single))(input)
}