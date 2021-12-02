use crate::common;

pub fn main(input: &str) -> String {
    let v = common::pairs::<String, i32>(input, " ");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[(String, i32)]) -> i32 {
    let mut h = 0;
    let mut d = 0;
    for (op, x) in v {
        match op.as_ref() {
            "forward" => h += x,
            "down" => d += x,
            "up" => d -= x,
            _ => (),
        }
    }
    d * h
}

fn find_b(v: &[(String, i32)]) -> i32 {
    let mut h = 0;
    let mut d = 0;
    let mut a = 0;
    for (op, x) in v {
        match op.as_ref() {
            "forward" => {
                h += x;
                d += a * x;
            }
            "down" => a += x,
            "up" => a -= x,
            _ => (),
        }
    }
    d * h
}
