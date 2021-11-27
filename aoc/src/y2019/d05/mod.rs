use crate::y2019::intcode::IntCode;

pub fn main(input: &str) -> String {
    let a = IntCode::run_until_end(input, &[1]);
    let b = IntCode::run_until_end(input, &[5]);
    format!("{} {}", a, b)
}
