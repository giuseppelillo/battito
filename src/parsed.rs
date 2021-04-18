use crate::parsed_measure::ParsedMeasure;
use crate::measure::Measure;

#[derive(Debug, PartialEq, Clone)]
pub enum Parsed {
    Group(Vec<ParsedMeasure>),
    Polymetric {
        elements: Vec<ParsedMeasure>,
        length: u32,
    },
}

pub enum Asd {
    Group(Vec<Asd>),
    Euclid { value: Asd, n: u32, m: u32, r: u32 },
    Repeat { value: Asd, n: u32 }, // [5 2]*2
    Replicate { value: Asd, n: u32 }, // [5 2]!3
}

impl Parsed {
    pub fn to_measures(&self) -> Vec<Measure> {
        todo!()
    }



}