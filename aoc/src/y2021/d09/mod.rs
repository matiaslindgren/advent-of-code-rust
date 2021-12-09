use crate::grid::{grid2d, Grid, Point};
use std::collections::{BinaryHeap, HashSet};

pub fn main(input: &str) -> String {
    let m = grid2d::<i32>(input);
    let a = find_a(&m);
    let b = find_b(&m);
    format!("{} {}", a, b)
}

type Map = Grid<i32>;

fn find_basins(m: &Map) -> Vec<(Point, i32)> {
    let is_minimum = |p, h1| m.adjacent(p).iter().all(|&(_, h2)| h1 < h2);
    m.iter().filter(|&(pos, h)| is_minimum(pos, h)).collect()
}

fn find_a(m: &Map) -> i32 {
    find_basins(m).iter().map(|(_, h)| h + 1).sum::<i32>()
}

fn find_b(m: &Map) -> i32 {
    let mut visited = HashSet::<Point>::new();
    let largest: BinaryHeap<i32> = find_basins(m)
        .iter()
        .map(|&(p, _)| basin_size(m, p, &mut visited))
        .collect();
    let n = largest.len();
    largest.into_sorted_vec().iter().skip(n - 3).product()
}

fn basin_size(m: &Map, p1: Point, visited: &mut HashSet<Point>) -> i32 {
    if visited.contains(&p1) {
        return 0;
    }
    visited.insert(p1);
    let h1 = m.get(p1).unwrap();
    if h1 == &9 {
        return 0;
    }
    let adjacent_size: i32 = m
        .adjacent(p1)
        .iter()
        .filter(|(_, h2)| h1 <= h2)
        .map(|&(p2, _)| basin_size(m, p2, visited))
        .sum();
    1 + adjacent_size
}
