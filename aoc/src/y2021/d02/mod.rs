use std::str;

pub fn main(input: &str) -> String {
    let v = parse_input(input);
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn parse_input(input: &str) -> Vec<(Op, i32)> {
    input
        .lines()
        .map(|line| {
            let (op, x) =
                line.split_once(' ').expect("expected line with one space");
            let op = op.parse::<Op>().expect("unknown op");
            let x = x.parse::<i32>().expect("expected number");
            (op, x)
        })
        .collect()
}

enum Op {
    Forward,
    Down,
    Up,
}
use Op::{Down, Forward, Up};

impl str::FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Forward),
            "down" => Ok(Down),
            "up" => Ok(Up),
            x => Err(format!("unknown up {}", x)),
        }
    }
}

fn find_a(v: &[(Op, i32)]) -> i32 {
    let mut h = 0;
    let mut d = 0;
    for (op, x) in v {
        match op {
            Forward => h += x,
            Down => d += x,
            Up => d -= x,
        }
    }
    d * h
}

fn find_b(v: &[(Op, i32)]) -> i32 {
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
    for (op, x) in v {
        match op {
            Forward => {
                h += x;
                d += a * x;
            }
            Down => a += x,
            Up => a -= x,
        }
    }
    d * h
}
