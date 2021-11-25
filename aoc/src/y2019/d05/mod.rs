use crate::common;
use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let v = common::numbers::<u32>(input, ",");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(_v: &[u32]) -> u32 {
    0
}

fn find_b(_v: &[u32]) -> u32 {
    0
}
