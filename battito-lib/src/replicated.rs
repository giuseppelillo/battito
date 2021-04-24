use crate::error::Error;
use crate::euclidean::Euclidean;
use crate::expansion::Expansion;
use crate::parsed_measure::ParsedMeasure;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map_res, map};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use crate::parser::{inner_parser_group, parser_note};
use crate::parser_alternate::parser_alternate;

pub struct Replicated {
    value: ParsedMeasure,
    replications: usize,
}

impl Expansion for Replicated {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        Ok(vec![
            self.value.clone();
            self.replications
        ])
    }

    fn parser(input: &str) -> IResult<&str, Self>
        where
            Self: Sized,
    {
        parser(input)
    }
}

fn parser(input: &str) -> IResult<&str, Replicated> {
    map_res(
        tuple((inner_parser, preceded(char('!'), digit1))),
        |(pm, digit)| -> Result<Replicated, Error> {
            let replications: Result<u32, Error> = digit.parse().map_err(Error::from);
            Ok(Replicated {
                value: pm,
                replications: digit.parse().unwrap(),
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
        parser_note,
        parser_alternate,
    ))(input)
}

