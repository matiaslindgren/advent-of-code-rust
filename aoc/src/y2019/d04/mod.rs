use crate::common;
use crate::math::decimal_digits;

pub fn main(input: &str) -> String {
    let v = common::items::<usize>(input, "-");
    let mut a = 0;
    let mut b = 0;
    for x in v[0]..=v[1] {
        let digits = decimal_digits(&x);
        let mut prev = digits[0];
        let mut repeats = 1;
        let mut has_two = false;
        let mut has_two_once = false;
        let mut is_monotonic = true;
        for &d in &digits[1..] {
            if d == prev {
                has_two = true;
                repeats += 1;
            } else {
                has_two_once |= repeats == 2;
                repeats = 1;
            }
            is_monotonic &= prev <= d;
            prev = d;
        }
        has_two_once |= repeats == 2;
        a += (has_two && is_monotonic) as usize;
        b += (has_two_once && is_monotonic) as usize;
    }
    format!("{} {}", a, b)
}
