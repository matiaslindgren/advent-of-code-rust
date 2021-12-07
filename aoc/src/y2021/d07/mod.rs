use crate::common;

pub fn main(input: &str) -> String {
    let crabs = common::items::<i32>(input, ",");
    let min_pos = *crabs.iter().min().unwrap();
    let max_pos = *crabs.iter().max().unwrap();
    let (a, b) =
        (min_pos..max_pos).fold((i32::MAX, i32::MAX), |(a, b), pos| {
            let a_delta = crabs.iter().map(|crab| (crab - pos).abs()).sum();
            let b_delta = crabs
                .iter()
                .map(|crab| {
                    let n = (crab - pos).abs();
                    n * (n + 1) / 2
                })
                .sum();
            (a.min(a_delta), b.min(b_delta))
        });
    format!("{} {}", a, b)
}
