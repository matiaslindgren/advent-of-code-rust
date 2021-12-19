use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

pub fn main(input: &str) -> String {
    let g = input.parse::<Grid>().unwrap();
    let dst = (g.h - 2, g.w - 2);
    let a = g.min_distance((1, 1), (dst.0 / 5, dst.1 / 5));
    let b = g.min_distance((1, 1), dst);
    format!("{} {}", a, b)
}

struct Grid {
    g: Vec<usize>,
    w: usize,
    h: usize,
}

type Point = (usize, usize);

impl Grid {
    fn new(w: usize, h: usize) -> Self {
        let (w, h) = (w + 2, h + 2);
        Self {
            g: vec![usize::MAX; w * h],
            w,
            h,
        }
    }

    fn get(&self, p: Point) -> usize {
        let (y, x) = p;
        self.g[y * self.w + x]
    }

    fn set(&mut self, p: Point, v: usize) {
        let (y, x) = p;
        self.g[y * self.w + x] = v;
    }

    fn min_distance(&self, src: Point, dst: Point) -> usize {
        let mut visited = HashSet::<Point>::new();
        let mut priority = BinaryHeap::<MinPriority>::new();
        let mut dist: HashMap<Point, usize> =
            self.points().iter().map(|&p| (p, usize::MAX)).collect();
        priority.push(MinPriority::new(src, 0));
        dist.insert(src, 0);
        while !priority.is_empty() {
            let current = priority.pop().unwrap();
            if visited.contains(&current.pos) {
                continue;
            }
            visited.insert(current.pos);
            for &adj in self.adjacent(current.pos).iter() {
                let src_to_cur = *dist.get(&current.pos).unwrap();
                let cur_to_adj = self.get(adj);
                let src_to_adj = dist.get_mut(&adj).unwrap();
                let src_to_adj_new = src_to_cur.saturating_add(cur_to_adj);
                if src_to_adj_new < *src_to_adj {
                    *src_to_adj = src_to_adj_new;
                    priority.push(MinPriority::new(adj, src_to_adj_new));
                }
            }
        }
        *dist.get(&dst).unwrap()
    }

    fn points(&self) -> Vec<Point> {
        (0..self.h)
            .flat_map(|y| (0..self.w).map(move |x| (y, x)))
            .collect()
    }

    fn adjacent(&self, p: Point) -> Vec<Point> {
        let (y, x) = p;
        vec![(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let h = s.lines().count();
        let w = s.len() / h;
        let mut g = Grid::new(5 * w, 5 * h);
        for (y, line) in s.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                g.set((y + 1, x + 1), (b - 48) as usize);
            }
        }
        let inc = |v| v % 9 + 1;
        for y in 1..=h {
            for x in (w + 1)..=(5 * w) {
                let p1 = (y, x - w);
                let p2 = (y, x);
                g.set(p2, inc(g.get(p1)));
            }
        }
        for x in 1..=(5 * w) {
            for y in (h + 1)..=(5 * h) {
                let p1 = (y - h, x);
                let p2 = (y, x);
                g.set(p2, inc(g.get(p1)));
            }
        }
        Ok(g)
    }
}

#[derive(Eq)]
struct MinPriority {
    dist: usize,
    pos:  Point,
}

impl MinPriority {
    fn new(pos: Point, dist: usize) -> Self {
        Self { dist, pos }
    }
}

impl Ord for MinPriority {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for MinPriority {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MinPriority {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}
