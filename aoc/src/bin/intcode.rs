use aoc::y2019::intcode::IntCode;
use std::env::args;

pub fn main() {
    let prog = args().nth(1).unwrap();
    let input = args().nth(2).unwrap().parse::<i64>().unwrap();
    let output = IntCode::run_until_end(&prog, &[input]);
    println!("{}", output);
}
