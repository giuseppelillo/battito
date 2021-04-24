use crate::error::{Error, ParsingError};
use crate::euclidean::Euclidean;
use crate::expansion::Expansion;
use crate::parsed_measure::{Parsed, ParsedMeasure, Polymetric};
use crate::parser_alternate::parser_alternate;
use crate::repeated::Repeated;
use crate::replicated::Replicated;
use crate::sequence::ParsedSequence;
use nom::combinator::map_res;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, digit1},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated, tuple},
    IResult,
};

pub fn parser_event_with_prob(input: &str) -> IResult<&str, ParsedMeasure> {
    map_res(
        tuple((alphanumeric1, preceded(char('?'), digit1))),
        |(value, prob): (&str, &str)| -> Result<ParsedMeasure, Error> {
            let prob_num: Result<u32, Error> = prob.parse().map_err(Error::from);
            Ok(ParsedMeasure::event_with_probability(value, prob_num?))
        },
    )(input)
}

pub fn parser_event_no_prob(input: &str) -> IResult<&str, ParsedMeasure> {
    map(alt((alphanumeric1, tag("~"))), ParsedMeasure::event)(input)
}

pub fn parser_event(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_event_with_prob, parser_event_no_prob))(input)
}

pub fn parser_single(input: &str) -> IResult<&str, ParsedMeasure> {
    alt((parser_event, parser_alternate))(input)
}

pub fn parser_parsed_measure(input: &str) -> IResult<&str, Vec<ParsedMeasure>> {
    alt((
        Repeated::parse,
        Replicated::parse,
        Euclidean::parse,
        map(inner_parser_group, |x| vec![x]),
        map(parser_single, |x| vec![x]),
    ))(input)
}

pub fn parser_group(input: &str) -> IResult<&str, ParsedMeasure> {
    map(separated_list0(char(' '), parser_parsed_measure), |v| {
        ParsedMeasure::Group(v.concat())
    })(input)
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
                elements: elements.concat(),
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
        Err(_) => Err(Error::DSLParsingError(ParsingError::Generic)),
    }
}
