use crate::error::Error;
use crate::parsed_measure::{Parsed, ParsedMeasure, Polymetric};
use crate::sequence::ParsedSequence;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn parser_alternate(input: &str) -> IResult<&str, ParsedMeasure> {
    map(
        preceded(
            char('<'),
            terminated(separated_list1(char(','), alphanumeric1), char('>')),
        ),
        ParsedMeasure::alternate,
    )(input)
}

fn parser_note(input: &str) -> IResult<&str, ParsedMeasure> {
    map(alt((alphanumeric1, tag("~"))), ParsedMeasure::note)(input)
}

fn parser_single(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_note, parser_alternate))(input)
}

fn parser_measure(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((inner_parser, parser_single))(input)
}

fn parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    map(
        separated_list0(char(' '), parser_measure),
        ParsedMeasure::Group,
    )(input)
}

fn parser_polymetric(input: &str) -> IResult<&str, Parsed> {
    map(
        tuple((
            preceded(
                char('{'),
                terminated(separated_list0(char(' '), parser_measure), char('}')),
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

fn parser_list(input: &str) -> IResult<&str, Parsed> {
    alt((parser_polymetric, map(parser_group, Parsed::ParsedMeasure)))(input)
}

fn parser_lists(input: &str) -> IResult<&str, Vec<Parsed>> {
    separated_list0(tag(" | "), parser_list)(input)
}

fn inner_parser(input: &str) -> IResult<&str, ParsedMeasure> {
    preceded(char('['), terminated(parser_group, char(']')))(input)
}

fn parser(input: &str) -> IResult<&str, ParsedSequence> {
    map(
        tuple((alphanumeric1, preceded(tag(" > "), parser_lists))),
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
