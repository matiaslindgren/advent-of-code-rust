use aoc::y2019::intcode::IntCode;
use std::env::args;

pub fn main() {
    let prog = args().nth(1).unwrap();
    let mut ic = IntCode::new(&prog);
    for input in args().skip(2) {
        ic.push_input(input.parse::<i64>().unwrap());
    }
    while !ic.done {
        if let Some(output) = ic.run() {
            println!("{}", output);
        }
    }
}
