use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<i32>(input, "\n");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[i32]) -> i32 {
    v.iter().fold(0, |s, x| s + x / 3 - 2)
}

fn find_b(v: &[i32]) -> i32 {
    let mut s = 0;
    for &x in v {
        let mut y = x;
        loop {
            y = y / 3 - 2;
            if y < 0 {
                break;
            }
            s += y;
        }
    }
    s
}
