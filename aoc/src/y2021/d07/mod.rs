use crate::common;

pub fn main(input: &str) -> String {
    let crabs = common::items::<i32>(input, ",");
    let min_pos = *crabs.iter().min().unwrap();
    let max_pos = *crabs.iter().max().unwrap();
    let (a, b) =
        (min_pos..=max_pos).fold((i32::MAX, i32::MAX), |(a, b), pos| {
            let deltas = crabs.iter().map(|crab| (crab - pos).abs());
            let a_delta = deltas.clone().sum();
            let b_delta = deltas.map(|d| d * (d + 1) / 2).sum();
            (a.min(a_delta), b.min(b_delta))
        });
    format!("{} {}", a, b)
}
