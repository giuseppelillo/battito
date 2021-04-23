pub mod error;
mod euclidean;
mod expansion;
pub mod interpreter;
pub mod max;
mod measure;
mod parsed_measure;
mod parser;
mod parser_alternate;
mod parser_euclidean;
mod primitives;
mod repeated;
mod sequence;
mod utils;

pub(crate) const VELOCITY_DEFAULT: u32 = 100;
pub(crate) const DURATION_DEFAULT: u32 = 100;
