use crate::primitives::{AlternateNote, Note};
use crate::measure::Measure;
use crate::utils::lcm_vec;
use crate::{VELOCITY_DEFAULT, DURATION_DEFAULT};

#[derive(Debug, PartialEq, Clone)]
pub enum Single {
    Note(Note),
    Alternate(AlternateNote),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ParsedMeasure {
    Single(Single),
    Group(Vec<ParsedMeasure>)
}

impl ParsedMeasure {

    // Transform this parsed measure into a vector of Measure
    pub fn to_measures(&self) -> Vec<Measure> {
        // expand polymetric
        println!("before expand polymetric {:?}", self);
        let expanded_polymetric = Self::expand_polymetric(self);
        println!("after expand polymetric {:?}", expanded_polymetric);
        // expand alternate
        Self::expand_alternate(expanded_polymetric)
    }
    fn expand_alternate(parsed_measures: Vec<ParsedMeasure>) -> Vec<Measure> {
        let mut out: Vec<Measure> = Vec::new();
        for pms in parsed_measures {
            let n = lcm_vec(pms.count_replications());
            // Create n copies of this ParsedMeasure
            let mut replicated: Vec<ParsedMeasure> = vec![pms.clone(); n as usize];
            Self::expand(&mut replicated);
            let expanded_alternate: Vec<Measure> =
                replicated.iter().map(|p| Self::out(p.clone())).collect();
            out.extend(expanded_alternate);
        }
        out
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

    fn next(v: &Vec<ParsedMeasure>, i: usize) -> ParsedMeasure {
        let index = i % v.len();
        v.get(index).unwrap().clone()
    }

    fn expand_polymetric(&self) -> Vec<ParsedMeasure> {
        match self {
            ParsedMeasure::Polymetric { elements, length } => {
                let mut out: Vec<ParsedMeasure> = Vec::with_capacity(elements.len());
                let mut i: usize = 0;
                for _ in 0..elements.len() {
                    let mut internal: Vec<ParsedMeasure> = Vec::with_capacity(*length as usize);
                    for _ in 0..*length {
                        internal.push(Self::next(elements, i));
                        i = i + 1;
                    }
                    out.push(ParsedMeasure::Group(internal));
                }
                out
            }
            _ => vec![self.clone()],
        }
    }

    fn out(parsed_measure: ParsedMeasure) -> Measure {
        match parsed_measure {
            Self::Single(Single::Note(n)) => Measure::Note(n.clone()),
            Self::Group(x) => {
                let nested: Vec<Measure> =
                    x.iter().map(|b| Self::out(b.clone())).collect();
                Measure::Group(nested)
            }
            _ => panic!("Not expected"),
        }
    }

    fn expand(replicated: &mut Vec<ParsedMeasure>) -> () {
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
            _ => (),
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
