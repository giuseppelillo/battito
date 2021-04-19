use crate::error::Error;
use crate::max::Payload;
use crate::parser::parse;

pub fn interpret(input: &str) -> Result<Payload, Error> {
    let parsed_sequence = parse(input)?;
    let sequence = parsed_sequence.to_sequence();
    Ok(sequence.to_max_message())
}
