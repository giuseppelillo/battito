use gcd::Gcd;

pub fn lcm_vec(v: Vec<u32>) -> u32 {
    v.iter().fold(1, |acc, i| lcm(acc, *i))
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / a.gcd(b)
}