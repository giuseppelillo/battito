use crate::euclidean::{Euclidean, EuclideanPrimitive};
use crate::parsed_measure::ParsedMeasure;
use crate::parser_alternate::parser_primitive;
use crate::primitives::PrimitiveGroup;
use nom::branch::alt;
use nom::character::complete::{alphanumeric1, char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub fn parser_euclidean_parsed_measure(input: &str) -> IResult<&str, ParsedMeasure> {
    map(parser_euclidean, |e| {
        let single = e.to_single_pattern().unwrap(); // TODO: use result
        ParsedMeasure::Single(single)
    })(input)
}

pub fn parser_euclidean(input: &str) -> IResult<&str, Euclidean> {
    map_res(tuple((parser_value, parser_numbers)), |value| {
        Euclidean::create(value.0, value.1 .0, value.1 .1, value.1 .2)
    })(input)
}

fn parser_value(input: &str) -> IResult<&str, PrimitiveGroup> {
    parser_primitive(input)
}

fn parser_numbers(
    input: &str,
) -> IResult<
    &str,
    (
        EuclideanPrimitive,
        EuclideanPrimitive,
        Option<EuclideanPrimitive>,
    ),
> {
    preceded(char('('), terminated(parser_numbers_inner, char(')')))(input)
}

fn parser_numbers_inner(
    input: &str,
) -> IResult<
    &str,
    (
        EuclideanPrimitive,
        EuclideanPrimitive,
        Option<EuclideanPrimitive>,
    ),
> {
    map_res(
        separated_list1(char(','), parser_euclidean_primitive),
        |x| {
            if x.len() == 3 {
                Ok((x[0].clone(), x[1].clone(), Some(x[2].clone())))
            } else if x.len() == 2 {
                Ok((x[0].clone(), x[1].clone(), None))
            } else {
                Err("Wrong parameters in euclidean")
            }
        },
    )(input)
}

fn parser_euclidean_primitive(input: &str) -> IResult<&str, EuclideanPrimitive> {
    alt((
        parser_euclidean_primitive_group,
        parser_euclidean_primitive_single,
    ))(input)
}

fn parser_euclidean_primitive_single(input: &str) -> IResult<&str, EuclideanPrimitive> {
    map(digit1, |x: &str| {
        let value: u32 = x.parse().unwrap();
        EuclideanPrimitive::Single(value)
    })(input)
}

fn parser_euclidean_primitive_group(input: &str) -> IResult<&str, EuclideanPrimitive> {
    map(
        preceded(
            char('<'),
            terminated(separated_list1(char(','), digit1), char('>')),
        ),
        |primitives: Vec<&str>| {
            EuclideanPrimitive::Alternate(primitives.iter().map(|x| x.parse().unwrap()).collect())
        },
    )(input)
}
