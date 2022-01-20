pub fn harmonic(fundamental: u8, grade: u8) -> u8 {
    fundamental * grade
}

pub struct Harmonic {
    fundamental: u32,
    grade: u32,
}

impl Harmonic {
    pub fn new(fundamental: u32) -> Harmonic {
        Harmonic { fundamental, grade: 0 }
    }
}

impl Iterator for Harmonic {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.grade += 1;
        Some(self.fundamental * self.grade)
    }
}

pub struct Binary {
    pub number: u32,
    pub length: u8,
}

impl Binary {
    pub fn generate(&self) -> Vec<u32> {
        format!("{:0>32b}", self.number)
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .skip(32 - self.length as usize)
            .collect()
    }
}

pub fn g(n: u8) -> u8 {
    if n == 0 {
        0
    } else {
        n - g(g(n - 1))
    }
}

pub fn h(n: u8) -> u8 {
    if n == 0 {
        0
    } else {
        n - h(h(h(n - 1)))
    }
}

#[cfg(test)]
mod tests {
    // use crate::generators::h;

    use super::Harmonic;

    #[test]
    fn test() {
        let mut iter = Harmonic::new(50);
        for _ in 0..20 {
            println!("{}", iter.next().unwrap())
        }
    }
}
