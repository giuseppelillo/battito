use self::{error::Error, parser::parse, pattern::Pattern, sequence::Sequence};

pub mod error;
mod measure;
mod parser;
pub mod pattern;
pub mod sequence;
mod utils;

pub const DEFAULT_SUBDIVISION: u32 = 1920;

pub fn transform(input: &str, subdivision: Option<u32>) -> Result<Pattern, Error> {
    let parsed_sequence = parse(input)?;
    let sequence = Sequence::from_parsed_sequence(&parsed_sequence, subdivision);
    Ok(sequence.to_pattern())
}

pub enum OutputFormat {
    Json,
    Max,
}
