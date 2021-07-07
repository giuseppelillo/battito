use crate::error::Error;
use crate::max::Payload;
use crate::parser::parse;

pub fn interpret(input: &str, run_config: &RunConfig) -> Result<Payload, Error> {
    let parsed_sequence = parse(input)?;
    let sequence = parsed_sequence.to_sequence(run_config.subdivision);
    Ok(sequence.to_max_message())
}

pub struct RunConfig {
    pub subdivision: u32,
}
