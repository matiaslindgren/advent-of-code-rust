use crate::common;

pub fn main(input: &str) -> String {
    let v = common::items::<i32>(input, "\n");
    let a = find_a(&v);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[i32]) -> i32 {
    v.iter()
        .zip(v[1..].iter())
        .fold(0, |n, (x1, x2)| n + (x1 < x2) as i32)
}

fn find_b(v: &[i32]) -> i32 {
    let mut n = 0;
    let mut s1 = v[0] + v[1] + v[2];
    for i in 3..v.len() {
        let s2 = s1 - v[i - 3] + v[i];
        n += (s1 < s2) as i32;
        s1 = s2;
    }
    n
}
