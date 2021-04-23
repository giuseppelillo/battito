use crate::error::EuclideanError::{NGreaterThanM, RGreaterEqualThanM};
use crate::error::{Error, ParsingError};
use crate::parsed_measure::{Single, ParsedMeasure};
use crate::primitives::Alternate;
use crate::primitives::{Note, PrimitiveGroup};
use crate::utils::lcm_vec;
use std::collections::VecDeque;
use crate::expansion::Expansion;
use nom::IResult;
use crate::parser_euclidean::parser_euclidean;

#[derive(Debug, PartialEq, Clone)]
pub enum EuclideanPrimitive {
    Single(u32),
    Alternate(Vec<u32>),
}

impl EuclideanPrimitive {
    pub fn next(&self, i: usize) -> u32 {
        match self {
            EuclideanPrimitive::Single(s) => *s,
            EuclideanPrimitive::Alternate(a) => {
                let index = i % a.len();
                *a.get(index).unwrap()
            }
        }
    }

    pub fn replications(&self) -> u32 {
        match self {
            EuclideanPrimitive::Single(_) => 1,
            EuclideanPrimitive::Alternate(x) => x.len() as u32,
        }
    }

    pub fn max_value(&self) -> Result<&u32, Error> {
        match self {
            EuclideanPrimitive::Single(s) => Ok(s),
            EuclideanPrimitive::Alternate(a) => a.iter().max().ok_or(Error::UnexpectedError),
        }
    }

    pub fn get_value(&self) -> Result<&u32, Error> {
        match self {
            EuclideanPrimitive::Single(v) => Ok(v),
            EuclideanPrimitive::Alternate(_) => Err(Error::UnexpectedError),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Euclidean {
    value: PrimitiveGroup,
    n: EuclideanPrimitive,
    m: EuclideanPrimitive,
    r: EuclideanPrimitive,
}

impl Euclidean {
    // b(<1,2,4>,<4,8>,<0,1>) -> <b(1,4,0),b(2,8,1),b(4,4,0),b(1,8,1),b(2,4,0),b(4,8,1)> -> <[b ~ ~ ~], [b
    pub fn to_single_pattern(&self) -> Result<Single, Error> {
        let alternates: Result<Vec<PrimitiveGroup>, Error> = self
            .expand_alternate()
            .iter()
            .map(|e| e.to_primitive_group())
            .collect();
        Ok(Single::Alternate(Alternate(alternates?)))
    }

    fn to_primitive_group(&self) -> Result<PrimitiveGroup, Error> {
        let steps = *self.m.get_value()?;
        let pulses = *self.n.get_value()?;
        let r = *self.r.get_value()? as usize;

        let mut pattern: Vec<PrimitiveGroup> = Vec::new();
        let mut counts: Vec<u32> = Vec::new();
        let mut remainders: Vec<u32> = Vec::new();
        let mut divisor = steps - pulses;
        remainders.push(pulses);
        let mut level: isize = 0;
        loop {
            counts.push(divisor / remainders[level as usize]);
            remainders.push(divisor % remainders[level as usize]);
            divisor = remainders[level as usize];
            level = level + 1;
            if remainders[level as usize] <= 1 {
                break;
            }
        }
        counts.push(divisor);

        fn build(
            euclidean: &Euclidean,
            level: isize,
            counts: &Vec<u32>,
            pattern: &mut Vec<PrimitiveGroup>,
            remainders: &Vec<u32>,
        ) -> () {
            match level {
                -1 => pattern.push(PrimitiveGroup::Single(Note::empty())),
                -2 => pattern.push(euclidean.value.clone()),
                _ => {
                    for _ in 0..counts[level as usize] {
                        build(euclidean, level - 1, counts, pattern, remainders);
                    }
                    if remainders[level as usize] != 0 {
                        build(euclidean, level - 2, counts, pattern, remainders);
                    }
                }
            }
        }

        build(self, level, &counts, &mut pattern, &remainders);
        let index_first_one = pattern
            .iter()
            .position(|x| *x != PrimitiveGroup::Single(Note::empty()))
            .unwrap();

        pattern.rotate_left(index_first_one);
        if steps - pulses == 1 {
            pattern.rotate_right(2);
        }

        pattern.rotate_right(r);
        Ok(PrimitiveGroup::Group(pattern))
    }

    fn count_replications(&self) -> [u32; 3] {
        [
            self.n.replications(),
            self.m.replications(),
            self.r.replications(),
        ]
    }

    fn expand_alternate(&self) -> Vec<Euclidean> {
        let n = lcm_vec(&self.count_replications());
        let mut replicated: Vec<Euclidean> = vec![self.clone(); n as usize];
        let mut i: usize = 0;
        for e in &mut replicated {
            *e = Euclidean {
                value: e.clone().value,
                n: EuclideanPrimitive::Single(e.n.next(i)),
                m: EuclideanPrimitive::Single(e.m.next(i)),
                r: EuclideanPrimitive::Single(e.r.next(i)),
            };
            i = i + 1;
        }
        replicated
    }

    pub fn create(
        value: PrimitiveGroup,
        n: EuclideanPrimitive,
        m: EuclideanPrimitive,
        r: Option<EuclideanPrimitive>,
    ) -> Result<Self, Error> {
        let r_unwrap = r.unwrap_or(EuclideanPrimitive::Single(0));
        if n.max_value()? > m.max_value()? {
            Err(Error::DSLParsingError(ParsingError::EuclideanError(
                NGreaterThanM,
            )))
        } else if r_unwrap.max_value()? >= m.max_value()? {
            Err(Error::DSLParsingError(ParsingError::EuclideanError(
                RGreaterEqualThanM,
            )))
        } else {
            Ok(Euclidean {
                value,
                n,
                m,
                r: r_unwrap,
            })
        }
    }
}

impl Expansion for Euclidean {
    fn expand(&self) -> Result<ParsedMeasure, Error> {
        let single = self.to_single_pattern()?;
        Ok(ParsedMeasure::Single(single))
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        parser_euclidean(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::euclidean::{Euclidean, EuclideanPrimitive};
    use crate::primitives::{Note, PrimitiveGroup};

    #[test]
    fn expansion() {
        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(3),
            m: EuclideanPrimitive::Alternate(vec![4, 8]),
            r: EuclideanPrimitive::Alternate(vec![0, 1, 2]),
        };
        let out = e.expand_alternate();
        let expected = vec![
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(4),
                r: EuclideanPrimitive::Single(0),
            },
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(8),
                r: EuclideanPrimitive::Single(1),
            },
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(4),
                r: EuclideanPrimitive::Single(2),
            },
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(8),
                r: EuclideanPrimitive::Single(0),
            },
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(4),
                r: EuclideanPrimitive::Single(1),
            },
            Euclidean {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclideanPrimitive::Single(3),
                m: EuclideanPrimitive::Single(8),
                r: EuclideanPrimitive::Single(2),
            },
        ];
        assert_eq!(expected, out);
    }

    #[test]
    fn transformation() {
        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(3),
            m: EuclideanPrimitive::Single(8),
            r: EuclideanPrimitive::Single(0),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());

        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(2),
            m: EuclideanPrimitive::Single(4),
            r: EuclideanPrimitive::Single(0),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());

        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(4),
            m: EuclideanPrimitive::Single(4),
            r: EuclideanPrimitive::Single(0),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());

        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(7),
            m: EuclideanPrimitive::Single(8),
            r: EuclideanPrimitive::Single(0),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());
    }

    #[test]
    fn transformation_nested() {
        let value = PrimitiveGroup::Group(vec![
            PrimitiveGroup::Group(vec![
                PrimitiveGroup::Single(Note {
                    value: "a".to_string(),
                    velocity: 120,
                    duration: 114,
                }),
                PrimitiveGroup::Single(Note {
                    value: "ll".to_string(),
                    velocity: 59,
                    duration: 63,
                }),
            ]),
            PrimitiveGroup::Single(Note {
                value: "b".to_string(),
                velocity: 100,
                duration: 100,
            }),
        ]);
        let e = Euclidean {
            value: value.clone(),
            n: EuclideanPrimitive::Single(2),
            m: EuclideanPrimitive::Single(4),
            r: EuclideanPrimitive::Single(0),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            value.clone(),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            value.clone(),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());
    }

    #[test]
    fn transformation_rotation() {
        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(3),
            m: EuclideanPrimitive::Single(8),
            r: EuclideanPrimitive::Single(1),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());

        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(2),
            m: EuclideanPrimitive::Single(4),
            r: EuclideanPrimitive::Single(2),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "0".to_string(),
                velocity: 0,
                duration: 0,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());

        let e = Euclidean {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclideanPrimitive::Single(4),
            m: EuclideanPrimitive::Single(4),
            r: EuclideanPrimitive::Single(3),
        };
        let expected = Ok(PrimitiveGroup::Group(vec![
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
        ]));
        assert_eq!(expected, e.to_primitive_group());
    }
}
