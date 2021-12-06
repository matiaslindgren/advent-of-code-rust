use crate::common;
use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let fish = common::items::<i32>(input, ",");
    let mut fc = FishCounter::default();
    let a: i64 = fish.iter().map(|&t| fc.fish_count(79, t)).sum();
    let b: i64 = fish.iter().map(|&t| fc.fish_count(255, t)).sum();
    format!("{} {}", a, b)
}

#[derive(Default)]
struct FishCounter {
    cache: HashMap<(i32, i32), i64>,
}

impl FishCounter {
    fn fish_count(&mut self, days: i32, timer: i32) -> i64 {
        match self.cache.get(&(days, timer)) {
            Some(count) => *count,
            None => {
                let count = {
                    if days < timer {
                        1
                    } else {
                        self.fish_count(days - timer, 7)
                            + self.fish_count(days - timer, 9)
                    }
                };
                self.cache.insert((days, timer), count);
                count
            }
        }
    }
}
