use std::io::Write;

use crate::pattern::parser::parsed_measure::{Parsed, ParsedMeasure, Single};

use super::primitives::{ParsedEvent, PrimitiveGroup};
const TAB: &str = "    ";

fn inden(level: u8) -> String {
    let mut b = "".to_owned();
    for _ in 0..level {
        b.push_str(TAB)
    }
    b
}

pub trait Fmt {
    fn fmt(&self, level: u8, b: &mut impl Write) -> std::io::Result<()>;
}

impl Fmt for ParsedEvent {
    fn fmt(&self, level: u8, buf: &mut impl Write) -> std::io::Result<()> {
        write!(buf, "{}Event: {} - {},\n", inden(level), self.value, self.probability)
    }
}

impl Fmt for PrimitiveGroup {
    fn fmt(&self, level: u8, b: &mut impl Write) -> std::io::Result<()> {
        match self {
            PrimitiveGroup::Single(e) => e.fmt(level, b),
            PrimitiveGroup::Group(g) => {
                write!(b, "{}PrimitiveGroup: [\n", inden(level))?;
                g.iter()
                    .map(|a| a.fmt(level + 1, b))
                    .collect::<std::io::Result<Vec<()>>>()?;
                write!(b, "{}],\n", inden(level))
            }
        }
    }
}

impl Fmt for Single {
    fn fmt(&self, level: u8, b: &mut impl Write) -> std::io::Result<()> {
        match self {
            Single::Event(e) => e.fmt(level, b),
            Single::Alternate(a) => {
                write!(b, "{}Alternate: [\n", inden(level))?;
                a.0.iter()
                    .map(|a| a.fmt(level + 1, b))
                    .collect::<std::io::Result<Vec<()>>>()?;
                write!(b, "{}],\n", inden(level))
            }
        }
    }
}

impl Fmt for ParsedMeasure {
    fn fmt(&self, level: u8, b: &mut impl Write) -> std::io::Result<()> {
        match self {
            ParsedMeasure::Single(s) => s.fmt(level, b),
            ParsedMeasure::Group(g) => {
                write!(b, "{}Group: [\n", inden(level))?;
                g.iter()
                    .map(|a| a.fmt(level + 1, b))
                    .collect::<std::io::Result<Vec<()>>>()?;
                write!(b, "{}],\n", inden(level))
            }
        }
    }
}

impl Fmt for Parsed {
    fn fmt(&self, level: u8, b: &mut impl Write) -> std::io::Result<()> {
        match self {
            Parsed::ParsedMeasure(p) => p.fmt(level, b),
            Parsed::Polymetric(p) => {
                write!(b, "{}Polymetric({}): [\n", inden(level), p.length)?;
                p.elements
                    .iter()
                    .map(|a| a.fmt(level + 1, b))
                    .collect::<std::io::Result<Vec<()>>>()?;
                write!(b, "{}],\n", inden(level))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::parser::{
        display::Fmt,
        parsed_measure::Single,
        primitives::{Alternate, ParsedEvent, PrimitiveGroup},
    };

    #[test]
    fn primitive() {
        let a = PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(ParsedEvent {
                value: "10".to_string(),
                probability: 100,
            }),
            PrimitiveGroup::Group(vec![
                PrimitiveGroup::Single(ParsedEvent {
                    value: "20".to_string(),
                    probability: 100,
                }),
                PrimitiveGroup::Single(ParsedEvent {
                    value: "30".to_string(),
                    probability: 100,
                }),
            ]),
        ]);
        let mut w = Vec::new();
        a.fmt(0, &mut w).unwrap();
        println!("{}", String::from_utf8_lossy(&w));
    }

    #[test]
    fn single() {
        let p = PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(ParsedEvent {
                value: "10".to_string(),
                probability: 100,
            }),
            PrimitiveGroup::Group(vec![
                PrimitiveGroup::Single(ParsedEvent {
                    value: "20".to_string(),
                    probability: 100,
                }),
                PrimitiveGroup::Single(ParsedEvent {
                    value: "30".to_string(),
                    probability: 100,
                }),
            ]),
        ]);
        let a = Single::Alternate(Alternate(vec![
            p,
            PrimitiveGroup::Single(ParsedEvent {
                value: "50".to_string(),
                probability: 100,
            }),
        ]));
        let mut w = Vec::new();
        a.fmt(0, &mut w).unwrap();
        println!("{}", String::from_utf8_lossy(&w));
    }
}
