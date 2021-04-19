use crate::error::Error;
use crate::parsed_measure::{Parsed, ParsedMeasure, Polymetric};
use crate::parser_alternate::parser_alternate;
use crate::sequence::ParsedSequence;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn parser_note(input: &str) -> IResult<&str, ParsedMeasure> {
    map(alt((alphanumeric1, tag("~"))), ParsedMeasure::note)(input)
}

pub fn parser_single(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_note, parser_alternate))(input)
}

pub fn parser_parsed_measure(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((inner_parser_group, parser_single))(input)
}

pub fn parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    map(
        separated_list0(char(' '), parser_parsed_measure),
        ParsedMeasure::Group,
    )(input)
}

pub fn parser_polymetric(input: &str) -> IResult<&str, Parsed> {
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
                elements,
                length: length.parse().unwrap(),
            })
        },
    )(input)
}

pub fn parser_measure(input: &str) -> IResult<&str, Parsed> {
    alt((parser_polymetric, map(parser_group, Parsed::ParsedMeasure)))(input)
}

pub fn parser_measures(input: &str) -> IResult<&str, Vec<Parsed>> {
    separated_list0(tag(" | "), parser_measure)(input)
}

pub fn inner_parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    preceded(char('['), terminated(parser_group, char(']')))(input)
}

pub fn parser(input: &str) -> IResult<&str, ParsedSequence> {
    map(
        tuple((alphanumeric1, preceded(tag(" > "), parser_measures))),
        |(target, measures)| ParsedSequence {
            target: target.to_string(),
            measures,
        },
    )(input)
}

pub fn parse(input: &str) -> Result<ParsedSequence, Error> {
    match parser(input) {
        Ok(e) => Ok(e.1),
        Err(_) => Err(Error::DSLParsingError),
    }
}
