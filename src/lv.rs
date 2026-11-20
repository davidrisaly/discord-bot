pub fn calc_exp(level: u32) -> u32 {
    100 * level.pow(2)
}

pub fn calc_level(exp: u32) -> u8 {
    (exp / 100u32).isqrt() as u8
}
