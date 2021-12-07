use crate::pattern::parser::parsed_measure::{ParsedMeasure, Single};
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct ParsedEvent {
    pub value: String,
    pub probability: u8, // [0, 100]
}

impl ParsedEvent {
    pub fn empty() -> Self {
        ParsedEvent {
            value: "0".to_string(),
            probability: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Alternate(pub Vec<PrimitiveGroup>);

impl Alternate {
    pub fn next(&self, i: usize) -> PrimitiveGroup {
        let index = i % self.0.len();
        self.0.get(index).unwrap().clone()
    }

    pub fn from_parsed_measures(pms: &Vec<ParsedMeasure>) -> Self {
        Alternate(pms.iter().map(PrimitiveGroup::from_parsed_measure).collect())
    }
}

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum PrimitiveGroup {
    Single(ParsedEvent),
    Group(Vec<PrimitiveGroup>),
}

impl PrimitiveGroup {
    pub fn to_parsed_measure(&self) -> ParsedMeasure {
        match self {
            PrimitiveGroup::Single(sp) => ParsedMeasure::Single(Single::Event(sp.clone())),
            PrimitiveGroup::Group(x) => ParsedMeasure::Group(x.iter().map(|pg| pg.to_parsed_measure()).collect()),
        }
    }

    pub fn from_parsed_measure(parsed_measure: &ParsedMeasure) -> Self {
        match parsed_measure {
            ParsedMeasure::Single(Single::Event(event)) => PrimitiveGroup::Single(event.clone()),
            ParsedMeasure::Group(x) => PrimitiveGroup::Group(x.iter().map(Self::from_parsed_measure).collect()),
            _ => panic!("Not expected here"),
        }
    }
}
