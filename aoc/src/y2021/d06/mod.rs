use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let fish: Vec<i32> = input
        .split(',')
        .map(|x| x.parse::<i32>().expect("failed parsing number"))
        .collect();
    let mut counter = MemoizedCounter::default();
    let a: i64 = fish.iter().map(|&t| counter.fish_count(79, t)).sum();
    let b: i64 = fish.iter().map(|&t| counter.fish_count(255, t)).sum();
    format!("{} {}", a, b)
}

#[derive(Default)]
struct MemoizedCounter {
    cache: HashMap<(i32, i32), i64>,
}

impl MemoizedCounter {
    fn fish_count(&mut self, days: i32, timer: i32) -> i64 {
        if let Some(&count) = self.cache.get(&(days, timer)) {
            count
        } else {
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
