use crate::max::{MaxNote, Pattern, Payload};
use crate::measure::Measure;
use crate::parsed_measure::Parsed;

#[derive(Debug, PartialEq)]
pub struct ParsedSequence {
    pub target: String,
    pub measures: Vec<Parsed>,
}

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub target: String,
    pub measures: Vec<Measure>,
}

impl ParsedSequence {
    pub fn to_sequence(&self) -> Sequence {
        Sequence {
            target: self.target.to_string(),
            measures: self.measures.iter().flat_map(|m| m.to_measures()).collect(),
        }
    }
}

impl Sequence {
    pub fn subdivision(&self) -> u32 {
        1920
    }

    pub fn to_max_message(&self) -> Payload {
        Payload {
            target: self.target.clone(),
            steps: self.to_pattern().serialize(),
            length: self.measures.len() as u32,
            subdivision: self.subdivision(),
        }
    }

    fn to_pattern(&self) -> Pattern {
        let mut pattern: Vec<MaxNote> = Vec::new();
        let mut i = 1;
        self.measures.iter().for_each(|m| {
            pattern.extend(m.to_pattern(i).0);
            i = i + self.subdivision();
        });

        Pattern(pattern)
    }
}
