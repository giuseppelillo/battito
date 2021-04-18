#[derive(Debug, PartialEq, Clone)]
pub struct Note {
    pub value: String,
    pub velocity: u32,
    pub duration: u32,
}

impl Note {
    pub fn advance(&self, index: u32, length: u32) -> u32 {
        let mut i = index;
        for _ in 1..length {
            i = i + 1;
        }
        i
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AlternateNote {
    pub notes: Vec<Note>,
}

impl AlternateNote {
    pub fn next(&self, i: usize) -> Note {
        let index = i % self.notes.len();
        self.notes.get(index).unwrap().clone()
    }
}
