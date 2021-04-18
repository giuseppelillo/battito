use crate::error::Error;
use crate::primitives::Note;
use nannou_osc::rosc::OscMessage;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct Payload {
    pub target: String,
    pub steps: String,
    pub length: u32,
    pub subdivision: u32,
}

impl Payload {
    pub fn to_osc_message(&self) -> Result<OscMessage, Error> {
        Ok(OscMessage {
            addr: serde_json::to_string(&self)?,
            args: None,
        })
    }
}

pub struct MaxNote {
    pub index: u32,
    pub note: Note,
}

impl MaxNote {
    pub fn display(&self) -> String {
        format!(
            "{} {} {} {}",
            self.index, self.note.value, self.note.velocity, self.note.duration
        )
    }
}

pub struct Pattern(pub Vec<MaxNote>);

impl Pattern {
    pub fn serialize(&self) -> String {
        let strings: Vec<String> = self.0.iter().map(|m| m.display()).collect();
        strings.join(", ")
    }
}
