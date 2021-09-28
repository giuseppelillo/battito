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
