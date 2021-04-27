use crate::error::Error;
use crate::euclidean::Euclidean;
use crate::expansion::Expansion;
use crate::parsed_measure::ParsedMeasure;
use crate::parser::alternate::parser_alternate;
use crate::parser::{inner_parser_group, parser_event};
use crate::replicated::Replicated;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub fn parser(input: &str) -> IResult<&str, Replicated> {
    map_res(
        tuple((inner_parser, preceded(char('!'), digit1))),
        |(pm, digit)| -> Result<Replicated, Error> {
            let replications: Result<u32, Error> = digit.parse().map_err(Error::from);
            Ok(Replicated {
                value: pm,
                replications: replications? as usize,
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
