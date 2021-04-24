use crate::max::{MaxEvent, Pattern};
use crate::measure::Measure::Group;
use crate::primitives::Event;
use crate::utils::lcm;

#[derive(Debug, PartialEq, Clone)]
pub enum Measure {
    Event(Event),
    Group(Vec<Measure>),
}

impl Measure {
    pub fn subdivision(&self) -> u32 {
        match self {
            Measure::Event(_) => 1,
            Measure::Group(elements) => Self::recurse_tree(1, 1, elements),
        }
    }

    pub fn to_pattern(&self, start: u32) -> Pattern {
        let max_events = match self {
            Measure::Event(event) => vec![MaxEvent {
                index: 1,
                event: event.clone(),
            }],
            Measure::Group(measures) => {
                let mut vec: Vec<MaxEvent> = Vec::new();
                Measure::max_event(1920, 1, &mut vec, start, measures);
                vec
            }
        };
        Pattern(max_events)
    }

    fn max_event(
        subdivision: u32,
        acc_value: u32,
        out: &mut Vec<MaxEvent>,
        index: u32,
        elements: &Vec<Measure>,
    ) -> u32 {
        let value = acc_value * elements.len() as u32;
        let length = subdivision / value;
        elements.iter().fold(index, |i, e| match e {
            Measure::Event(event) => {
                let max_event = MaxEvent {
                    index: i,
                    event: event.clone(),
                };
                let new_i = event.advance(i, length);
                out.push(max_event);
                new_i + 1
            }
            Group(xs) => Measure::max_event(subdivision, value, out, i, xs),
        })
    }

    fn recurse_tree(acc_value: u32, acc_lcm: u32, elements: &Vec<Measure>) -> u32 {
        let value = acc_value * elements.len() as u32;
        elements.iter().fold(acc_lcm, |l, e| match e {
            Measure::Event(_) => lcm(l, value),
            Measure::Group(xs) => Self::recurse_tree(value, l, xs),
        })
    }
}
