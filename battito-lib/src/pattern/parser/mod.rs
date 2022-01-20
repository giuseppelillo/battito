mod alternate;
pub(crate) mod display;
mod euclidean;
mod expansion;
pub(crate) mod parsed_measure;
pub(crate) mod primitives;
mod repeated;
mod replicated;

use crate::generator::Binary;

use self::expansion::harmonic_generator::HarmonicGenerator;
use self::parsed_measure::{Parsed, ParsedMeasure, Polymetric};

use super::error::{Error, ParsingError};
use super::parser::alternate::parser_alternate;
use super::parser::expansion::euclidean::Euclidean;
use super::parser::expansion::repeated::Repeated;
use super::parser::expansion::replicated::Replicated;
use super::parser::expansion::Expansion;
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct ParsedSequence {
    pub measures: Vec<Parsed>,
    pub length: Option<u32>,
}

pub fn parse(input: &str) -> Result<ParsedSequence, Error> {
    match parser(input) {
        Ok(e) => Ok(e.1),
        Err(_) => Err(Error::DSLParsingError(ParsingError::Generic)),
    }
}

fn parser_event_with_prob(input: &str) -> IResult<&str, ParsedMeasure> {
    map_res(
        tuple((alphanumeric1, preceded(char('?'), digit1))),
        |(value, prob): (&str, &str)| -> Result<ParsedMeasure, Error> {
            let prob_num: Result<u8, Error> = prob.parse().map_err(Error::from);
            Ok(ParsedMeasure::event_with_probability(value, prob_num?))
        },
    )(input)
}

fn parser_event_no_prob(input: &str) -> IResult<&str, ParsedMeasure> {
    map(alt((alphanumeric1, tag("~"))), ParsedMeasure::event)(input)
}

fn parser_event(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_event_with_prob, parser_event_no_prob))(input)
}

fn parser_single(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_event, parser_alternate))(input)
}

fn parser_parsed_measure(input: &str) -> IResult<&str, Vec<ParsedMeasure>> {
    alt((
        HarmonicGenerator::parse,
        Binary::parse,
        Repeated::parse,
        Replicated::parse,
        Euclidean::parse,
        map(inner_parser_group, |x| vec![x]),
        map(parser_single, |x| vec![x]),
    ))(input)
}

fn parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    map(separated_list0(char(' '), parser_parsed_measure), |v| {
        ParsedMeasure::Group(v.concat())
    })(input)
}

fn parser_polymetric(input: &str) -> IResult<&str, Parsed> {
    map(
        tuple((
            preceded(
                char('{'),
                terminated(separated_list0(char(' '), parser_parsed_measure), char('}')),
            ),
            preceded(char('%'), digit1),
        )),
        |(elements, length)| {
            Parsed::Polymetric(Polymetric {
                elements: elements.concat(),
                length: length.parse().unwrap(),
            })
        },
    )(input)
}

fn parser_measure(input: &str) -> IResult<&str, Parsed> {
    alt((parser_polymetric, map(parser_group, Parsed::ParsedMeasure)))(input)
}

fn parser_measures(input: &str) -> IResult<&str, (Vec<Parsed>, &str)> {
    map(separated_list0(tag(" | "), parser_measure), |p| (p, input))(input)
}

fn inner_parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    preceded(char('['), terminated(parser_group, char(']')))(input)
}

fn parser(input: &str) -> IResult<&str, ParsedSequence> {
    map(
        tuple((parser_measures, opt(preceded(tag(" / "), digit1)))),
        |(parsed, length)| ParsedSequence {
            measures: parsed.0,
            length: length.map(|s| s.parse().unwrap()),
        },
    )(input)
}
