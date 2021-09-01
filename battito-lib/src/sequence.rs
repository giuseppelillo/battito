use crate::max::{MaxEvent, Pattern, Payload};
use crate::measure::Measure;
use crate::parsed_measure::Parsed;
use crate::SUBDIVISION;

#[derive(Debug, PartialEq)]
pub struct ParsedSequence {
    pub target: String,
    pub measures: Vec<Parsed>,
    pub pattern: String,
    pub length: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub target: String,
    pub measures: Vec<Measure>,
    pub subdivision: u32,
    pub pattern: String,
    pub length: u32,
}

impl ParsedSequence {
    pub fn to_sequence(&self) -> Sequence {
        Sequence::new(
            self.target.to_string(),
            &self.measures.iter().flat_map(|m| m.to_measures()).collect(),
            self.pattern.clone(),
            self.length,
        )
    }
}

impl Sequence {
    pub fn new(target: String, measures: &Vec<Measure>, pattern: String, length: Option<u32>) -> Sequence {
        Sequence {
            target,
            measures: measures.clone(),
            pattern,
            subdivision: SUBDIVISION,
            length: match length {
                Some(l) if l > measures.len() as u32 => l,
                _ => measures.len() as u32,
            }
        }
    }
    pub fn to_max_message(&self) -> Payload {
        Payload {
            target: self.target.clone(),
            steps: self.to_pattern().serialize(),
            // length: (self.measures.len() as u32) * self.length,
            length: self.length,
            pattern: self.pattern.clone(),
        }
    }

    fn to_pattern(&self) -> Pattern {
        let mut pattern: Vec<MaxEvent> = Vec::new();
        let mut i = 1;
        self.measures.iter().for_each(|m| {
            pattern.extend(m.to_pattern(i, self.length as f32 / self.measures.len() as f32).0);
            i = i + self.subdivision;
        });

        Pattern(pattern)
    }
}
