use crate::parsed_measure::{ParsedMeasure, Single};

#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub value: String,
    pub velocity: u32,
    pub duration: u32,
}

impl Note {
    pub fn advance(&self, index: u32, length: u32) -> u32 {
        let mut i = index;
        for _ in 1..length {
            i = i + 1;
        }
        i
    }

    pub fn empty() -> Self {
        Note {
            value: "0".to_string(),
            velocity: 0,
            duration: 0,
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
        Alternate(
            pms.iter()
                .map(PrimitiveGroup::from_parsed_measure)
                .collect(),
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimitiveGroup {
    Single(Note),
    Group(Vec<PrimitiveGroup>),
}

impl PrimitiveGroup {
    pub fn to_parsed_measure(&self) -> ParsedMeasure {
        match self {
            PrimitiveGroup::Single(sp) => ParsedMeasure::Single(Single::Note(sp.clone())),
            PrimitiveGroup::Group(x) => {
                ParsedMeasure::Group(x.iter().map(|pg| pg.to_parsed_measure()).collect())
            }
        }
    }

    pub fn from_parsed_measure(parsed_measure: &ParsedMeasure) -> Self {
        match parsed_measure {
            ParsedMeasure::Single(Single::Note(note)) => PrimitiveGroup::Single(note.clone()),
            ParsedMeasure::Group(x) => {
                PrimitiveGroup::Group(x.iter().map(Self::from_parsed_measure).collect())
            }
            _ => panic!("Not expected here"),
        }
    }
}
