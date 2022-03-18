use std::collections::{BinaryHeap, HashSet};
use std::str;

pub fn main(input: &str) -> String {
    let map = input.parse::<Map>().expect("failed parsing map");
    let a = find_a(&map);
    let b = find_b(&map);
    format!("{} {}", a, b)
}

fn find_a(map: &Map) -> u32 {
    map.find_basins()
        .into_iter()
        .map(|pos| map.depth(pos) + 1)
        .sum::<u32>()
}

fn find_b(map: &Map) -> u32 {
    let mut visited = HashSet::<Vec2>::new();
    let largest: BinaryHeap<u32> = map
        .find_basins()
        .into_iter()
        .map(|p| map.basin_size(p, &mut visited))
        .collect();
    let n = largest.len();
    largest.into_sorted_vec().iter().skip(n - 3).product()
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

struct Map {
    map:   Vec<u32>,
    width: i64,
}

impl str::FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = vec![];
        let mut width = 0;
        for line in s.lines() {
            width = line.len() as i64;
            for ch in line.chars() {
                let height = ch
                    .to_digit(10)
                    .ok_or(format!("failed parsing digit '{}'", ch))?;
                map.push(height);
            }
        }
        Ok(Self { map, width })
    }
}

impl Map {
    fn find_basins(&self) -> Vec<Vec2> {
        (0..self.map.len())
            .map(|i| Vec2 {
                x: (i as i64) % self.width,
                y: (i as i64) / self.width,
            })
            .filter(|&p1| {
                self.adjacent(p1)
                    .into_iter()
                    .all(|p2| self.depth(p1) < self.depth(p2))
            })
            .collect()
    }

    fn depth(&self, p: Vec2) -> u32 {
        self.map[(p.y * self.width + p.x) as usize]
    }

    fn adjacent(&self, p1: Vec2) -> Vec<Vec2> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .filter_map(|(dx, dy)| {
                let p2 = Vec2 {
                    x: p1.x + dx,
                    y: p1.y + dy,
                };
                let w = self.width;
                let h = (self.map.len() as i64) / w;
                let is_inside = 0 <= p2.x && p2.x < w && 0 <= p2.y && p2.y < h;
                if is_inside {
                    Some(p2)
                } else {
                    None
                }
            })
            .collect()
    }

    fn basin_size(&self, p1: Vec2, visited: &mut HashSet<Vec2>) -> u32 {
        if visited.contains(&p1) {
            return 0;
        }
        visited.insert(p1);
        if self.depth(p1) == 9 {
            return 0;
        }
        let adjacent_size: u32 = self
            .adjacent(p1)
            .into_iter()
            .filter_map(|p2| {
                if self.depth(p1) <= self.depth(p2) {
                    Some(self.basin_size(p2, visited))
                } else {
                    None
                }
            })
            .sum();
        1 + adjacent_size
    }
}
