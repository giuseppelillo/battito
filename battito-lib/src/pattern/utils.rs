use gcd::Gcd;

pub fn lcm_vec<'a, I>(v: I) -> u32
where
    I: IntoIterator<Item = &'a u32>,
{
    v.into_iter().fold(1, |acc, i| lcm(acc, *i))
}

pub fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / a.gcd(b)
}
