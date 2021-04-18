use crate::max::{MaxNote, Pattern, Payload};
use crate::measure::Measure;
use crate::parsed_measure::ParsedMeasure;
use crate::parsed::Parsed;

#[derive(Debug, PartialEq)]
pub struct ParsedSequence {
    pub target: String,
    pub measures: Vec<Parsed>,
}

#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub target: String,
    pub measures: Vec<Measure>,
}

impl ParsedSequence {
    pub fn to_sequence(&self) -> Sequence {
        Sequence {
            target: self.target.to_string(),
            measures: self.measures.iter().flat_map(|m| m.to_measures()).collect(),
        }
    }
}

impl Sequence {
    pub fn subdivision(&self) -> u32 {
        1920
    }

    pub fn to_max_message(&self) -> Payload {
        Payload {
            target: self.target.clone(),
            steps: self.to_pattern().serialize(),
            length: self.measures.len() as u32,
            subdivision: self.subdivision(),
        }
    }

    fn to_pattern(&self) -> Pattern {
        let mut pattern: Vec<MaxNote> = Vec::new();
        let mut i = 1;
        self.measures.iter().for_each(|m| {
            pattern.extend(m.to_pattern(i).0);
            i = i + self.subdivision();
        });

        Pattern(pattern)
    }
}


#[cfg(test)]
mod tests {
    use crate::sequence::Measure;
    use crate::sequence::Measure::Group;

    #[test]
    fn three_level_nesting() {
        // [[60 61 [62 63]] 65 [66 67] 68]
        let elements = Group(vec![
            Group(vec![
                Measure::note("60"),
                Measure::note("61"),
                Group(vec![Measure::note("62"), Measure::note("63")]),
            ]),
            Measure::note("65"),
            Group(vec![Measure::note("66"), Measure::note("67")]),
            Measure::note("68"),
        ]);
        println!("{}", elements.generate_max_notes(1));
        assert_eq!(24, elements.subdivision());
    }

    #[test]
    fn three_level_nesting_shuffled() {
        // [68 65 [66 67] [60 61 [62 63]]]
        let elements = Group(vec![
            Measure::note("68"),
            Measure::note("65"),
            Group(vec![Measure::note("66"), Measure::note("67")]),
            Group(vec![
                Measure::note("60"),
                Measure::note("61"),
                Group(vec![Measure::note("62"), Measure::note("63")]),
            ]),
        ]);
        assert_eq!(24, elements.subdivision());
    }

    #[test]
    fn odd_nesting() {
        // [1 2 [3 0] 4 [5 0 1] 6 7]
        let elements = Group(vec![
            Measure::note("61"),
            Measure::note("62"),
            Group(vec![Measure::note("73"), Measure::note("70")]),
            Measure::note("64"),
            Group(vec![
                Measure::note("65"),
                Measure::note("60"),
                Measure::note("61"),
            ]),
            Measure::note("66"),
            Measure::note("67"),
        ]);
        assert_eq!(42, elements.subdivision());
        println!("{}", elements.generate_max_notes(1));
    }

    #[test]
    fn odd_deeply_nested() {
        // [[0 1 [2 3 4 [5 6 8 9 0]]] 2]
        let elements = Group(vec![
            Group(vec![
                Measure::note("0"),
                Measure::note("1"),
                Group(vec![
                    Measure::note("3"),
                    Measure::note("4"),
                    Group(vec![
                        Measure::note("5"),
                        Measure::note("6"),
                        Measure::note("8"),
                        Measure::note("9"),
                        Measure::note("0"),
                    ]),
                ]),
            ]),
            Group(vec![Measure::note("2"), Measure::note("3")]),
            Measure::note("3"),
        ]);
        assert_eq!(270, elements.subdivision());
    }
}
