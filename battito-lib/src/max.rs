use crate::primitives::Event;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Payload {
    pub target: String,
    pub steps: String,
    pub length: u32,
    pub subdivision: u32,
}

pub struct MaxEvent {
    pub index: u32,
    pub event: Event,
}

impl MaxEvent {
    pub fn display(&self) -> String {
        format!(
            "{} {} {} {}",
            self.index, self.event.value, self.event.velocity, self.event.duration
        )
    }
}

pub struct Pattern(pub Vec<MaxEvent>);

impl Pattern {
    pub fn serialize(&self) -> String {
        let strings: Vec<String> = self.0.iter().map(|m| m.display()).collect();
        strings.join(", ")
    }
}
