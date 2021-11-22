use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<u32>(input, "\n");
    format!("{} {}", find_a(&v), find_b(&v))
}

fn find_a(v: &[u32]) -> u32 {
    let mut s: u32 = 0;
    for x in v.iter() {
        s += x / 3 - 2;
    }
    s
}

fn find_b(v: &[u32]) -> u32 {
    let mut s: u32 = 0;
    for x in v.iter() {
        let mut y = x.to_owned() as i32;
        loop {
            y = y / 3 - 2;
            if y < 0 {
                break;
            }
            s += y as u32;
        }
    }
    s
}
