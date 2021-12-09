use crate::common::range;
use crate::grid::{Grid, Point};

pub fn main(input: &str) -> String {
    let points: Vec<(Point, Point)> = parse_points(input);
    let mut g1 = Grid::<i64>::new(0, 0);
    let mut g2 = Grid::<i64>::new(0, 0);
    for &(l, r) in points.iter() {
        let (x1, y1) = l;
        let (x2, y2) = r;
        if x1 == x2 || y1 == y2 {
            for y in y1.min(y2)..=y1.max(y2) {
                for x in x1.min(x2)..=x1.max(x2) {
                    let pos = (y, x);
                    let c = 1 + *g1.get_default(pos);
                    g1.set(pos, c);
                    let c = 1 + *g2.get_default(pos);
                    g2.set(pos, c);
                }
            }
        } else {
            for (&y, x) in range(y1, y2, 1).iter().zip(range(x1, x2, 1)) {
                let c = 1 + *g2.get_default((y, x));
                g2.set((y, x), c);
            }
        }
    }
    let a = g1.g.values().filter(|c| **c > 1).count();
    let b = g2.g.values().filter(|c| **c > 1).count();
    format!("{} {}", a, b)
}

fn parse_points(input: &str) -> Vec<(Point, Point)> {
    let parse_point = |(x, y): (&str, &str)| {
        (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
    };
    input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" -> ").unwrap();
            let p1 = l.split_once(',').unwrap();
            let p2 = r.split_once(',').unwrap();
            (parse_point(p1), parse_point(p2))
        })
        .collect()
}
