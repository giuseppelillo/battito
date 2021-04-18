use crate::measure::Measure;
use crate::primitives::{AlternateNote, Note};
use crate::utils::lcm_vec;
use crate::{DURATION_DEFAULT, VELOCITY_DEFAULT};

#[derive(Debug, PartialEq, Clone)]
pub enum Single {
    Note(Note),
    Alternate(AlternateNote),
}

/*pub struct Euclid {
    n: Single,
    m: Single,
    r: Single,
}*/

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
        // Self::expand_alternate(vec![self.clone()])
        let n = lcm_vec(self.count_replications());
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
            ParsedMeasure::Single(Single::Alternate(x)) => acc.push(x.notes.len() as u32),
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
            Self::Single(Single::Note(n)) => Measure::Note(n.clone()),
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
            Self::rec(pm, i);
            i = i + 1;
        }
    }

    fn rec(pm: &mut ParsedMeasure, iter: usize) -> () {
        match pm {
            ParsedMeasure::Single(Single::Note(_)) => (),
            ParsedMeasure::Single(Single::Alternate(an)) => {
                *pm = ParsedMeasure::Single(Single::Note(an.next(iter)))
            }
            ParsedMeasure::Group(x) => {
                for a in x {
                    Self::rec(a, iter);
                }
            }
        }
    }

    // Constructors
    pub fn alternate(value: Vec<&str>) -> Self {
        let notes: Vec<Note> = value
            .iter()
            .map(|value| {
                let (value_parsed, velocity, duration) = match *value {
                    "~" => ("0", 0, 0),
                    p => (p, VELOCITY_DEFAULT, DURATION_DEFAULT),
                };
                Note {
                    value: value_parsed.to_string(),
                    velocity,
                    duration,
                }
            })
            .collect();

        Self::Single(Single::Alternate(AlternateNote { notes }))
    }

    pub fn note(value: &str) -> Self {
        let (value_parsed, velocity, duration) = match value {
            "~" => ("0", 0, 0),
            p => (p, VELOCITY_DEFAULT, DURATION_DEFAULT),
        };
        Self::Single(Single::Note(Note {
            value: value_parsed.to_string(),
            velocity,
            duration,
        }))
    }

    pub fn note_pitch_velocity(value: &str, velocity: u32) -> Self {
        let value_parsed = match value {
            "~" => "0",
            p => p,
        };
        Self::Single(Single::Note(Note {
            value: value_parsed.to_string(),
            velocity,
            duration: DURATION_DEFAULT,
        }))
    }

    pub fn note_pitch_duration(value: &str, duration: u32) -> Self {
        let value_parsed = match value {
            "~" => "0",
            p => p,
        };
        Self::Single(Single::Note(Note {
            value: value_parsed.to_string(),
            velocity: VELOCITY_DEFAULT,
            duration,
        }))
    }

    pub fn note_pitch_velocity_duration(value: &str, velocity: u32, duration: u32) -> Self {
        let value_parsed = match value {
            "~" => "0",
            p => p,
        };
        Self::Single(Single::Note(Note {
            value: value_parsed.to_string(),
            velocity,
            duration,
        }))
    }
}

impl Polymetric {
    // Transform this parsed measure into a vector of Measure
    pub fn to_measures(&self) -> Vec<Measure> {
        let group = ParsedMeasure::Group(self.elements.clone());
        let n = lcm_vec(group.count_replications());
        // Create n copies of this ParsedMeasure
        let mut replicated: Vec<ParsedMeasure> = vec![group; n as usize];
        Self::expand_alternate(&mut replicated);
        println!("replicated: {:?}", replicated);
        let extracted_and_flattened: Vec<ParsedMeasure> = Self::extract_and_flatten(replicated);
        println!("flattened: {:?}", extracted_and_flattened);
        let expanded_polymetric: Vec<ParsedMeasure> = Self::expand_polymetric(&extracted_and_flattened, self.length);
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
            ParsedMeasure::Single(Single::Note(_)) => (),
            ParsedMeasure::Single(Single::Alternate(an)) => {
                *pm = ParsedMeasure::Single(Single::Note(an.next(iter)))
            }
            ParsedMeasure::Group(x) => {
                for a in x {
                    Self::rec(a, iter);
                }
            }
        }
    }

    fn expand_polymetric(elements: &Vec<ParsedMeasure>, length: u32) -> Vec<ParsedMeasure> {
        let mut out: Vec<ParsedMeasure> = Vec::with_capacity(elements.len());
        let mut i: usize = 0;
        for _ in 0..elements.len() {
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
            ParsedMeasure::Single(Single::Note(n)) => Measure::Note(n.clone()),
            ParsedMeasure::Group(x) => {
                let nested: Vec<Measure> = x.iter().map(|b| Self::out(b.clone())).collect();
                Measure::Group(nested)
            }
            _ => panic!("Not expected"),
        }
    }
}
