use crate::max::{MaxEvent, Pattern, Payload};
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
    pub subdivision: u32,
}

impl ParsedSequence {
    pub fn to_sequence(&self, subdivision: u32) -> Sequence {
        Sequence {
            target: self.target.to_string(),
            measures: self.measures.iter().flat_map(|m| m.to_measures()).collect(),
            subdivision,
        }
    }
}

impl Sequence {
    pub fn to_max_message(&self, velocity: u32, duration: u32) -> Payload {
        Payload {
            target: self.target.clone(),
            steps: self.to_pattern().serialize(velocity, duration),
            length: self.measures.len() as u32,
            subdivision: self.subdivision,
        }
    }

    fn to_pattern(&self) -> Pattern {
        let mut pattern: Vec<MaxEvent> = Vec::new();
        let mut i = 1;
        self.measures.iter().for_each(|m| {
            pattern.extend(m.to_pattern(i).0);
            i = i + self.subdivision;
        });

        Pattern(pattern)
    }
}
