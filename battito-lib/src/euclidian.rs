use crate::parsed_measure::Single;
use crate::primitives::{Note, PrimitiveGroup};
use crate::utils::lcm_vec;

#[derive(Debug, PartialEq, Clone)]
pub enum EuclidianPrimitive {
    Single(u32),
    Alternate(Vec<u32>),
}

impl EuclidianPrimitive {
    pub fn next(&self, i: usize) -> u32 {
        match self {
            EuclidianPrimitive::Single(s) => *s,
            EuclidianPrimitive::Alternate(a) => {
                let index = i % a.len();
                *a.get(index).unwrap()
            }
        }
    }

    pub fn replications(&self) -> u32 {
        match self {
            EuclidianPrimitive::Single(_) => 1,
            EuclidianPrimitive::Alternate(x) => x.len() as u32,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Euclidian {
    value: PrimitiveGroup,
    n: EuclidianPrimitive,
    m: EuclidianPrimitive,
    r: EuclidianPrimitive,
}

impl Euclidian {
    // b(<1,2,4>,<4,8>,<0,1>) -> <b(1,4,0),b(2,8,1),b(4,4,0),b(1,8,1),b(2,4,0),b(4,8,1)>
    pub fn to_single_pattern(&self) -> Single {
        todo!()
    }

    fn count_replications(&self) -> [u32; 3] {
        [
            self.n.replications(),
            self.m.replications(),
            self.r.replications(),
        ]
    }

    fn expand_alternate(&self) -> Vec<Euclidian> {
        let n = lcm_vec(&self.count_replications());
        let mut replicated: Vec<Euclidian> = vec![self.clone(); n as usize];
        let mut i: usize = 0;
        for e in &mut replicated {
            *e = Euclidian {
                value: e.clone().value,
                n: EuclidianPrimitive::Single(e.n.next(i)),
                m: EuclidianPrimitive::Single(e.m.next(i)),
                r: EuclidianPrimitive::Single(e.r.next(i)),
            };
            i = i + 1;
        }
        replicated
    }

    pub fn create(
        value: PrimitiveGroup,
        n: EuclidianPrimitive,
        m: EuclidianPrimitive,
        r: Option<EuclidianPrimitive>,
    ) -> Self {
        Euclidian {
            value,
            n,
            m,
            r: r.unwrap_or(EuclidianPrimitive::Single(0)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::euclidian::EuclidianPrimitive::Single as euclidian_single;
    use crate::euclidian::{Euclidian, EuclidianPrimitive};
    use crate::primitives::PrimitiveGroup::Single as primitive_single;
    use crate::primitives::{Note, PrimitiveGroup};

    #[test]
    fn exploration() {
        let e = Euclidian {
            value: PrimitiveGroup::Single(Note {
                value: "x".to_string(),
                velocity: 100,
                duration: 100,
            }),
            n: EuclidianPrimitive::Single(3),
            m: EuclidianPrimitive::Alternate(vec![4, 8]),
            r: EuclidianPrimitive::Alternate(vec![0, 1, 2]),
        };
        let out = e.expand_alternate();
        let expected = vec![
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(4),
                r: EuclidianPrimitive::Single(0),
            },
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(8),
                r: EuclidianPrimitive::Single(1),
            },
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(4),
                r: EuclidianPrimitive::Single(2),
            },
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(8),
                r: EuclidianPrimitive::Single(0),
            },
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(4),
                r: EuclidianPrimitive::Single(1),
            },
            Euclidian {
                value: PrimitiveGroup::Single(Note {
                    value: "x".to_string(),
                    velocity: 100,
                    duration: 100,
                }),
                n: EuclidianPrimitive::Single(3),
                m: EuclidianPrimitive::Single(8),
                r: EuclidianPrimitive::Single(2),
            },
        ];
        assert_eq!(expected, out);
    }
}
