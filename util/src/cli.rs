use std::env::args;

const USAGE: &str = "usage: aoc year day";

fn parse(nth: usize) -> u32 {
    let input = args().nth(nth).expect(USAGE);
    match str::parse::<u32>(&input) {
        Ok(x) => x,
        Err(_) => panic!("{}", USAGE),
    }
}

pub fn parse_args() -> (u32, u32) {
    (parse(1), parse(2))
}
