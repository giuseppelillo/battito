use crate::error::Error;
use crate::euclidean::Euclidean;
use crate::expansion::Expansion;
use crate::parsed_measure::ParsedMeasure;
use crate::parser::{inner_parser_group, parser_event};
use crate::parser_alternate::parser_alternate;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub struct Repeated {
    value: ParsedMeasure,
    repetitions: usize,
}

impl Expansion for Repeated {
    fn expand(&self) -> Result<Vec<ParsedMeasure>, Error> {
        Ok(vec![ParsedMeasure::Group(vec![self.value.clone(); self.repetitions])])
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