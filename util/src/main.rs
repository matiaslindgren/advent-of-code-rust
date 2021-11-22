mod input;
mod output;

use std::env::args;

const USAGE: &str = "usage: aoc year day";

fn parse(nth: usize) -> u32 {
    let input = args().nth(nth).expect(USAGE);
    match str::parse::<u32>(&input) {
        Ok(x) => x,
        Err(_) => panic!("{}", USAGE),
    }
}

fn main() {
    let year = parse(1);
    let day = parse(2);
    let input = input::get(year, day);
    let output = output::get(&input, year, day);
    println!("{}", output);
}
