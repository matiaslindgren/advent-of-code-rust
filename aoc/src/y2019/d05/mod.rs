use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let (_, a) = intcode::run(input, &[1]);
    let (_, b) = intcode::run(input, &[5]);
    format!("{} {}", a, b)
}
