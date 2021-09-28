use super::alternate::parser_primitive;
use super::expansion::euclidean::{Euclidean, EuclideanPrimitive};
use super::primitives::PrimitiveGroup;
use crate::pattern::error::Error;
use nom::branch::alt;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

pub(crate) fn parser_euclidean(input: &str) -> IResult<&str, Euclidean> {
    map_res(tuple((parser_value, parser_numbers)), |value| {
        Euclidean::create(value.0, value.1 .0, value.1 .1, value.1 .2)
    })(input)
}

fn parser_value(input: &str) -> IResult<&str, PrimitiveGroup> {
    parser_primitive(input)
}

fn parser_numbers(input: &str) -> IResult<&str, (EuclideanPrimitive, EuclideanPrimitive, Option<EuclideanPrimitive>)> {
    preceded(char('('), terminated(parser_numbers_inner, char(')')))(input)
}

fn parser_numbers_inner(
    input: &str,
) -> IResult<&str, (EuclideanPrimitive, EuclideanPrimitive, Option<EuclideanPrimitive>)> {
    map_res(separated_list1(char(','), parser_euclidean_primitive), |x| {
        if x.len() == 3 {
            Ok((x[0].clone(), x[1].clone(), Some(x[2].clone())))
        } else if x.len() == 2 {
            Ok((x[0].clone(), x[1].clone(), None))
        } else {
            Err("Wrong parameters in euclidean")
        }
    })(input)
}

fn parser_euclidean_primitive(input: &str) -> IResult<&str, EuclideanPrimitive> {
    alt((parser_euclidean_primitive_group, parser_euclidean_primitive_single))(input)
}

fn parser_euclidean_primitive_single(input: &str) -> IResult<&str, EuclideanPrimitive> {
    map_res(digit1, |x: &str| -> Result<EuclideanPrimitive, Error> {
        let value: Result<u32, Error> = x.parse().map_err(Error::from);
        Ok(EuclideanPrimitive::Single(value?))
    })(input)
}

fn parser_euclidean_primitive_group(input: &str) -> IResult<&str, EuclideanPrimitive> {
    map(
        preceded(char('<'), terminated(separated_list1(char(','), digit1), char('>'))),
        |primitives: Vec<&str>| EuclideanPrimitive::Alternate(primitives.iter().map(|x| x.parse().unwrap()).collect()),
    )(input)
}
