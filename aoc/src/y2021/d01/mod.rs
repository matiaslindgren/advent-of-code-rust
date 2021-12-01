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
        .fold(0, |n, (x0, x1)| n + (x0 < x1) as i32)
}

fn find_b(v: &[i32]) -> i32 {
    let mut s = v[0] + v[1] + v[2];
    let mut sums: Vec<i32> = vec![s];
    sums.extend(v.iter().zip(v[3..].iter()).map(|(x0, x3)| {
        s += -x0 + x3;
        s
    }));
    find_a(&sums)
}
