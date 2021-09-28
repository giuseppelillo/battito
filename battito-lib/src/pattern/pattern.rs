use crate::pattern::measure::Event;
use serde::Serialize;
use serde_json::json;

use super::OutputFormat;

#[derive(Debug, PartialEq, Serialize)]
pub struct Payload {
    pub steps: String,
    pub length: u32,
}

// Event associated to a particular time index
#[derive(Debug, PartialEq, Serialize)]
pub struct TimedEvent {
    pub index: u32,
    pub event: Event,
}

impl TimedEvent {
    pub fn max_format(&self) -> String {
        format!("{} {} {}", self.index, self.event.value, self.event.probability)
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Pattern {
    pub steps: Vec<TimedEvent>,
    pub length: u32,
    pub subdivision: u32,
}

impl Pattern {
    pub fn format_steps(&self, output_format: OutputFormat) -> String {
        match output_format {
            OutputFormat::Json => serde_json::to_string_pretty(self).unwrap(),
            OutputFormat::Max => {
                let strings: Vec<String> = self.steps.iter().map(|e| e.max_format()).collect();
                strings.join(", ")
                // json!({
                //     "steps": strings.join(", "),
                //     "length": self.length,
                //     "subdivision": self.subdivision
                // })
                // .to_string()
            }
        }
    }
}
