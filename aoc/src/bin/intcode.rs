use aoc::y2019::intcode::run;
use std::env::args;

pub fn main() {
    let prog = args().nth(1).unwrap();
    let input = args().nth(2).unwrap().parse::<i64>().unwrap();
    let (_, output) = run(&prog, &[input]);
    println!("{}", output);
}
