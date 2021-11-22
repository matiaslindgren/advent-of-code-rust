mod io;
mod solutions;

use std::env::args;

const USAGE: &str = "usage: aoc year day [level-to-POST]";

fn parse(nth: usize, required: bool) -> u32 {
    let input = args().nth(nth).unwrap_or_default();
    match str::parse::<u32>(&input) {
        Ok(x) => x,
        Err(_) => {
            if required {
                panic!("{}", USAGE);
            }
            0
        }
    }
}

fn main() {
    let year = parse(1, true);
    let day = parse(2, true);
    let input = io::get(year, day);
    let output = solutions::solve(&input, year, day);
    println!("{}", output);
    let level = parse(3, false);
    match level {
        0 => (),
        1 | 2 => io::post(year, day, level, &output),
        level => panic!("cannot post unknown level {}", level),
    }
}
