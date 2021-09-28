use crate::pattern::parser::parser_event;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{preceded, terminated};
use nom::IResult;

use super::parsed_measure::{ParsedMeasure, Single};
use super::primitives::{Alternate, PrimitiveGroup};

pub(crate) fn parser_alternate(input: &str) -> IResult<&str, ParsedMeasure> {
    map(
        preceded(
            char('<'),
            terminated(separated_list1(char(','), parser_primitive), char('>')),
        ),
        |primitives| ParsedMeasure::Single(Single::Alternate(Alternate(primitives))),
    )(input)
}

fn parser_primitive_event(input: &str) -> IResult<&str, PrimitiveGroup> {
    map(parser_event, |pm| PrimitiveGroup::from_parsed_measure(&pm))(input)
}

fn parser_primitive_group(input: &str) -> IResult<&str, PrimitiveGroup> {
    preceded(char('['), terminated(parser_group_inner, char(']')))(input)
}

pub(crate) fn parser_group_inner(input: &str) -> IResult<&str, PrimitiveGroup> {
    map(separated_list0(char(' '), parser_primitive), |x| {
        PrimitiveGroup::Group(x)
    })(input)
}

pub(crate) fn parser_primitive(input: &str) -> IResult<&str, PrimitiveGroup> {
    alt((parser_primitive_group, parser_primitive_event))(input)
}
