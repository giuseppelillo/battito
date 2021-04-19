use crate::max::{MaxNote, Pattern};
use crate::measure::Measure::Group;
use crate::primitives::Note;
use crate::utils::lcm;

#[derive(Debug, PartialEq, Clone)]
pub enum Measure {
    Note(Note),
    Group(Vec<Measure>),
}

impl Measure {
    pub fn subdivision(&self) -> u32 {
        match self {
            Measure::Note(_) => 1,
            Measure::Group(elements) => Self::recurse_tree(1, 1, elements),
        }
    }

    pub fn to_pattern(&self, start: u32) -> Pattern {
        let max_notes = match self {
            Measure::Note(note) => vec![MaxNote {
                index: 1,
                note: note.clone(),
            }],
            Measure::Group(measures) => {
                let mut vec: Vec<MaxNote> = Vec::new();
                Measure::max_notes(1920, 1, &mut vec, start, measures);
                vec
            }
        };
        Pattern(max_notes)
    }

    fn max_notes(
        subdivision: u32,
        acc_value: u32,
        out: &mut Vec<MaxNote>,
        index: u32,
        elements: &Vec<Measure>,
    ) -> u32 {
        let value = acc_value * elements.len() as u32;
        let length = subdivision / value;
        elements.iter().fold(index, |i, e| match e {
            Measure::Note(note) => {
                let max_note = MaxNote {
                    index: i,
                    note: note.clone(),
                };
                let new_i = note.advance(i, length);
                out.push(max_note);
                new_i + 1
            }
            Group(xs) => Measure::max_notes(subdivision, value, out, i, xs),
        })
    }

    fn recurse_tree(acc_value: u32, acc_lcm: u32, elements: &Vec<Measure>) -> u32 {
        let value = acc_value * elements.len() as u32;
        elements.iter().fold(acc_lcm, |l, e| match e {
            Measure::Note(_) => lcm(l, value),
            Measure::Group(xs) => Self::recurse_tree(value, l, xs),
        })
    }
}
