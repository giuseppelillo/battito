use crate::pattern::measure::Measure::Group;
use crate::pattern::pattern::{Pattern, TimedEvent};
use crate::pattern::utils::lcm;
use serde::Serialize;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct Event {
    pub value: String,
    pub probability: u32, // [0, 100]
}

impl Event {
    pub fn advance(&self, index: u32, length: u32) -> u32 {
        let mut i = index;
        for _ in 1..length {
            i = i + 1;
        }
        i
    }
}

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

    pub fn timed_events(&self, start: u32, length_multiplier: f32, subdivision: u32) -> Vec<TimedEvent> {
        let timed_events = match self {
            Measure::Event(event) if event.probability != 0 => vec![TimedEvent {
                index: 1,
                event: event.clone(),
            }],
            Measure::Group(measures) => {
                let mut vec: Vec<TimedEvent> = Vec::new();
                Measure::timed_event(subdivision, 1, &mut vec, start, measures, length_multiplier);
                vec
            }
            _ => vec![],
        };
        timed_events
    }

    fn timed_event(
        subdivision: u32,
        acc_value: u32,
        out: &mut Vec<TimedEvent>,
        index: u32,
        elements: &Vec<Measure>,
        length_multiplier: f32,
    ) -> u32 {
        let value = acc_value * elements.len() as u32;
        let length = subdivision / value;
        elements.iter().fold(index, |i, e| match e {
            Measure::Event(event) if event.probability != 0 => {
                // println!("length multiplier: {}", length_multiplier);
                let timed_event = TimedEvent {
                    index: ((i - 1) as f32 * length_multiplier) as u32 + 1,
                    event: event.clone(),
                };
                let new_i = event.advance(i, length);
                out.push(timed_event);
                new_i + 1
            }
            Measure::Event(event) => event.advance(i, length) + 1,
            Group(xs) => Measure::timed_event(subdivision, value, out, i, xs, length_multiplier),
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
