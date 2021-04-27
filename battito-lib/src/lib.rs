pub mod interpreter;
mod parser;
mod utils;
pub mod sequence;
pub mod max;
pub mod euclidean;
pub mod expansion;
pub mod error;
pub mod primitives;
mod measure;
pub mod parsed_measure;
pub mod repeated;
pub mod replicated;

pub(crate) const VELOCITY_DEFAULT: u32 = 100;
pub(crate) const DURATION_DEFAULT: u32 = 100;
