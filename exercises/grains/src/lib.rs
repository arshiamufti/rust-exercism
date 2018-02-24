pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    2u64.pow(s-1)
}

/*
 * the sum of the first 64 terms of the geometric series is 1, 2, 4,... is
 * 2^(64) - 1
 */
pub fn total() -> u64 {
    std::u64::MAX
}
