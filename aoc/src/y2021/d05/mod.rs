use std::collections::HashMap;
use std::num;
use std::str;

pub fn main(input: &str) -> String {
    let points: Vec<(Vec2, Vec2)> = parse_points(input);
    let a = find_a(&points);
    let b = find_b(&points);
    format!("{} {}", a, b)
}

fn find_a(points: &[(Vec2, Vec2)]) -> usize {
    count_overlapping(points).counts()
}

fn find_b(points: &[(Vec2, Vec2)]) -> usize {
    let mut overlapping = count_overlapping(points);
    for (p1, p2) in points.iter().filter(|&(p1, p2)| p1.diagonal_to(p2)) {
        let y_range = range(p1.y, p2.y);
        let x_range = range(p1.x, p2.x);
        for (y, x) in y_range.into_iter().zip(x_range.into_iter()) {
            overlapping.inc(y, x);
        }
    }
    overlapping.counts()
}

fn count_overlapping(points: &[(Vec2, Vec2)]) -> OverlapCounter {
    let mut overlapping = OverlapCounter::default();
    for (p1, p2) in points.iter().filter(|&(p1, p2)| !p1.diagonal_to(p2)) {
        let y_min = p1.y.min(p2.y);
        let y_max = p1.y.max(p2.y);
        let x_min = p1.x.min(p2.x);
        let x_max = p1.x.max(p2.x);
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                overlapping.inc(y, x);
            }
        }
    }
    overlapping
}

fn range(a: i64, b: i64) -> Vec<i64> {
    if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    }
}

#[derive(Default, Clone)]
struct OverlapCounter {
    counts: HashMap<Vec2, i64>,
}

impl OverlapCounter {
    fn inc(&mut self, y: i64, x: i64) {
        let pos = Vec2 { y, x };
        if let Some(n) = self.counts.get_mut(&pos) {
            *n += 1;
        } else {
            self.counts.insert(pos, 1);
        }
    }

    fn counts(&self) -> usize {
        self.counts.values().filter(|&&c| c > 1).count()
    }
}

fn parse_points(input: &str) -> Vec<(Vec2, Vec2)> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line
                .split_once(" -> ")
                .expect("points should be separated by ' -> '");
            let p1 = l.parse::<Vec2>().expect("failed parsing left point");
            let p2 = r.parse::<Vec2>().expect("failed parsing right point");
            (p1, p2)
        })
        .collect()
}

#[derive(Clone, PartialEq, Hash, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

impl str::FromStr for Vec2 {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) =
            s.split_once(',').expect("point should be separated by ','");
        let x = x.parse::<i64>()?;
        let y = y.parse::<i64>()?;
        Ok(Self { x, y })
    }
}

impl Vec2 {
    fn diagonal_to(&self, other: &Self) -> bool {
        self.x != other.x && self.y != other.y
    }
}
