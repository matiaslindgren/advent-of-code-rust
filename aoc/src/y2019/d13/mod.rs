use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<u32>(input, ",");
    format!("{} {}", find_a(&v), find_b(&v))
}

fn find_a(_v: &[u32]) -> u32 {
    0
}

fn find_b(_v: &[u32]) -> u32 {
    0
}
