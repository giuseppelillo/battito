use crate::display::Fmt;
use crate::error::Error;
use crate::max::Payload;
use crate::parser::parse;

pub fn interpret(input: &str) -> Result<Payload, Error> {
    let parsed_sequence = parse(input)?;
    let mut w = Vec::new();
    parsed_sequence.measures[0].fmt(0, &mut w).unwrap();
    println!("{}", String::from_utf8_lossy(&w));
    // println!("{:?}", parsed_sequence);
    let sequence = parsed_sequence.to_sequence();
    println!("{:?}", sequence);
    println!("{:?}", sequence.to_max_message());
    Ok(sequence.to_max_message())
}
