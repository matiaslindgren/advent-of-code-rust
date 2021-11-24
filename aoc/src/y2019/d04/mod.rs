use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<usize>(input, "-");
    let mut a = 0;
    let mut b = 0;
    for x in v[0]..=v[1] {
        let digits = common::decimal_digits(&x);
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
                if repeats == 2 {
                    has_two_once = true;
                }
                repeats = 1;
            }
            is_monotonic &= prev <= d;
            prev = d;
        }
        if repeats == 2 {
            has_two_once = true;
        }
        a += (has_two && is_monotonic) as usize;
        b += (has_two_once && is_monotonic) as usize;
    }
    format!("{} {}", a, b)
}
