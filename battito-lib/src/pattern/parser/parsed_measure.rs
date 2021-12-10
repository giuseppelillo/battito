use crate::pattern::measure::{self, Measure};
use crate::pattern::utils::lcm_vec;

use super::primitives::{Alternate, ParsedEvent};

#[derive(Debug, PartialEq, Clone)]
pub enum Single {
    Event(ParsedEvent),
    Alternate(Alternate),
}

impl Single {
    pub fn empty() -> Self {
        Single::Event(ParsedEvent {
            value: "0".to_string(),
            probability: 0,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polymetric {
    pub elements: Vec<ParsedMeasure>,
    pub length: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Parsed {
    ParsedMeasure(ParsedMeasure),
    Polymetric(Polymetric),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedMeasure {
    Single(Single),
    Group(Vec<ParsedMeasure>),
}

impl Parsed {
    pub fn to_measures(&self) -> Vec<Measure> {
        match self {
            Parsed::ParsedMeasure(parsed_measure) => parsed_measure.to_measures(),
            Parsed::Polymetric(polymetric) => polymetric.to_measures(),
        }
    }
}

impl ParsedMeasure {
    // Transform this parsed measure into a vector of Measure
    pub fn to_measures(&self) -> Vec<Measure> {
        let n = lcm_vec(&self.count_replications());
        // Create n copies of this ParsedMeasure
        let mut replicated: Vec<ParsedMeasure> = vec![self.clone(); n as usize];
        Self::expand_alternate(&mut replicated);
        replicated.iter().map(|p| Self::out(p.clone())).collect()
    }

    fn count_replications(&self) -> Vec<u32> {
        let mut reps: Vec<u32> = Vec::new();
        Self::_count_replications(&mut reps, self);
        reps
    }

    fn _count_replications(acc: &mut Vec<u32>, p: &ParsedMeasure) -> () {
        match p {
            ParsedMeasure::Single(Single::Alternate(x)) => acc.push(x.0.len() as u32),
            ParsedMeasure::Group(pms) => {
                for i in pms {
                    Self::_count_replications(acc, i)
                }
            }
            _ => (),
        }
    }

    fn out(parsed_measure: ParsedMeasure) -> Measure {
        match parsed_measure {
            Self::Single(Single::Event(n)) => Measure::Event(measure::Event {
                value: n.value.clone(),
                probability: n.probability,
            }),
            Self::Group(x) => {
                let nested: Vec<Measure> = x.iter().map(|b| Self::out(b.clone())).collect();
                Measure::Group(nested)
            }
            _ => panic!("Not expected"),
        }
    }

    fn expand_alternate(replicated: &mut Vec<ParsedMeasure>) -> () {
        // Remove Alternate, Polymetric
        let mut i: usize = 0;
        for pm in replicated {
            Self::expand_rec(pm, i);
            i = i + 1;
        }
    }

    fn expand_rec(pm: &mut ParsedMeasure, iter: usize) -> () {
        match pm {
            ParsedMeasure::Single(Single::Event(_)) => (),
            ParsedMeasure::Single(Single::Alternate(an)) => *pm = an.next(iter).to_parsed_measure(),
            ParsedMeasure::Group(x) => {
                for a in x {
                    Self::expand_rec(a, iter);
                }
            }
        }
    }

    // Constructors
    pub fn alternate(value: Vec<(&str, Option<u8>)>) -> Self {
        let events: Vec<ParsedMeasure> = value
            .iter()
            .map(|value| {
                let (value_parsed, probability) = match *value {
                    ("~", _) => ("0", Some(0)),
                    (v, p) => (v, p),
                };
                Self::Single(Single::Event(ParsedEvent {
                    value: value_parsed.to_string(),
                    probability: probability.unwrap_or(100),
                }))
            })
            .collect();

        Self::Single(Single::Alternate(Alternate::from_parsed_measures(&events)))
    }

    pub fn event(value: &str) -> Self {
        let (value_parsed, probability) = match value {
            "~" => ("0", 0),
            p => (p, 100),
        };
        Self::Single(Single::Event(ParsedEvent {
            value: value_parsed.to_string(),
            probability,
        }))
    }

    pub fn event_with_probability(value: &str, probability: u8) -> Self {
        let (value_parsed, prob) = match value {
            "~" => ("0", 0),
            p => (p, probability),
        };
        Self::Single(Single::Event(ParsedEvent {
            value: value_parsed.to_string(),
            probability: prob,
        }))
    }
}

impl Polymetric {
    // Transform this parsed measure into a vector of Measure
    pub fn to_measures(&self) -> Vec<Measure> {
        let group = ParsedMeasure::Group(self.elements.clone());
        let n = lcm_vec(&group.count_replications());
        // Create n copies of this ParsedMeasure
        let mut replicated: Vec<ParsedMeasure> = vec![group; n as usize];
        Self::expand_alternate(&mut replicated);
        let extracted_and_flattened: Vec<ParsedMeasure> = Self::extract_and_flatten(replicated);
        let expanded_polymetric: Vec<ParsedMeasure> =
            Self::expand_polymetric(&extracted_and_flattened, self.length as usize);
        expanded_polymetric.iter().map(|p| Self::out(p.clone())).collect()
    }

    // [Group(x,y,z), Group(a,b,c)] => [x,y,z,a,b,c]
    fn extract_and_flatten(elements: Vec<ParsedMeasure>) -> Vec<ParsedMeasure> {
        let mut out: Vec<ParsedMeasure> = Vec::new();
        for i in elements {
            match i {
                ParsedMeasure::Single(_) => out.push(i),
                ParsedMeasure::Group(x) => out.extend(x),
            }
        }
        out
    }

    fn expand_alternate(replicated: &mut Vec<ParsedMeasure>) -> () {
        // Remove Alternate
        let mut i: usize = 0;
        for pm in replicated {
            Self::rec(pm, i);
            i = i + 1;
        }
    }

    fn rec(pm: &mut ParsedMeasure, iter: usize) -> () {
        match pm {
            ParsedMeasure::Single(Single::Event(_)) => (),
            ParsedMeasure::Single(Single::Alternate(an)) => *pm = an.next(iter).to_parsed_measure(),
            ParsedMeasure::Group(x) => {
                for a in x {
                    Self::rec(a, iter);
                }
            }
        }
    }

    fn expand_polymetric(elements: &Vec<ParsedMeasure>, length: usize) -> Vec<ParsedMeasure> {
        let elements_len = elements.len();
        let number_of_measures = if elements_len % length != 0 && length % elements_len != 0 {
            elements_len
        } else {
            if elements_len <= length {
                1
            } else {
                elements_len / length
            }
        };
        let mut out: Vec<ParsedMeasure> = Vec::with_capacity(number_of_measures);
        let mut i: usize = 0;
        for _ in 0..number_of_measures {
            let mut internal: Vec<ParsedMeasure> = Vec::with_capacity(length as usize);
            for _ in 0..length {
                internal.push(Self::next(elements, i));
                i = i + 1;
            }
            out.push(ParsedMeasure::Group(internal));
        }
        out
    }

    fn next(v: &Vec<ParsedMeasure>, i: usize) -> ParsedMeasure {
        let index = i % v.len();
        v.get(index).unwrap().clone()
    }

    fn out(parsed_measure: ParsedMeasure) -> Measure {
        match parsed_measure {
            ParsedMeasure::Single(Single::Event(n)) => Measure::Event(measure::Event {
                value: n.value.clone(),
                probability: n.probability,
            }),
            ParsedMeasure::Group(x) => {
                let nested: Vec<Measure> = x.iter().map(|b| Self::out(b.clone())).collect();
                Measure::Group(nested)
            }
            _ => panic!("Not expected"),
        }
    }
}
