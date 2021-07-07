pub mod error;
pub mod euclidean;
pub mod expansion;
pub mod interpreter;
pub mod max;
mod measure;
pub mod parsed_measure;
mod parser;
pub mod primitives;
pub mod repeated;
pub mod replicated;
pub mod sequence;
mod utils;

pub const SUBDIVISION_DEFAULT: u32 = 1920;
