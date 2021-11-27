use crate::common;

pub fn main(input: &str) -> String {
    let v = common::items::<u32>(input, "\n");
    format!("{} {}", find_a(&v), find_b(&v))
}

fn find_a(v: &[u32]) -> u32 {
    for x in v.iter() {
        for y in v.iter() {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

fn find_b(v: &[u32]) -> u32 {
    for x in v.iter() {
        for y in v.iter() {
            for z in v.iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    0
}
