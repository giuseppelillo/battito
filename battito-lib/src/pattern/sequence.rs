use crate::pattern::measure::Measure;
use crate::pattern::pattern::{Pattern, TimedEvent};
use crate::pattern::DEFAULT_SUBDIVISION;

use super::parser::ParsedSequence;

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub measures: Vec<Measure>,
    pub subdivision: u32,
    pub length: u32,
}

impl Sequence {
    pub fn from_parsed_sequence(parsed_sequence: &ParsedSequence, subdivision: Option<u32>) -> Sequence {
        Sequence::new(
            &parsed_sequence.measures.iter().flat_map(|m| m.to_measures()).collect(),
            parsed_sequence.length,
            subdivision,
        )
    }
    fn new(measures: &Vec<Measure>, length: Option<u32>, subdivision: Option<u32>) -> Sequence {
        Sequence {
            measures: measures.clone(),
            subdivision: subdivision.unwrap_or(DEFAULT_SUBDIVISION),
            length: match length {
                Some(l) if l > measures.len() as u32 => l,
                _ => measures.len() as u32,
            },
        }
    }

    pub fn to_pattern(&self) -> Pattern {
        let mut steps: Vec<TimedEvent> = Vec::new();
        let mut i = 1;
        self.measures.iter().for_each(|m| {
            steps.extend(m.timed_events(i, self.length as f32 / self.measures.len() as f32, self.subdivision));
            i = i + self.subdivision;
        });

        Pattern {
            steps,
            length: self.length,
            subdivision: self.subdivision,
        }
    }
}
