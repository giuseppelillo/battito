use super::alternate::parser_alternate;
use super::expansion::euclidean::Euclidean;
use super::expansion::repeated::Repeated;
use super::expansion::Expansion;
use super::parsed_measure::ParsedMeasure;
use super::{inner_parser_group, parser_event};
use crate::pattern::error::Error;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub fn parser(input: &str) -> IResult<&str, Repeated> {
    map_res(
        tuple((inner_parser, preceded(char('*'), digit1))),
        |(pm, digit)| -> Result<Repeated, Error> {
            let repetitions: Result<u32, Error> = digit.parse().map_err(Error::from);
            Ok(Repeated {
                value: pm,
                repetitions: repetitions? as usize,
            })
        },
    )(input)
}

fn inner_parser(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((inner_parser_group, parser_single))(input)
}

fn parser_single(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((
        map(Euclidean::parse, |v| v.first().unwrap().clone()),
        parser_event,
        parser_alternate,
    ))(input)
}
