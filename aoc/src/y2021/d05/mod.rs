use crate::common::range;
use crate::grid::{Grid, Point};

pub fn main(input: &str) -> String {
    let v: Vec<(Point, Point)> = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(" -> ").unwrap();
            let p1 = l.split_once(",").unwrap();
            let p2 = r.split_once(",").unwrap();
            (parse_point(p1), parse_point(p2))
        })
        .collect();
    let mut g1 = Grid::<i64>::new(0, 0);
    let mut g2 = Grid::<i64>::new(0, 0);
    for &(l, r) in v.iter() {
        let (x1, y1) = l;
        let (x2, y2) = r;
        let y_range: Vec<i64> = (y1.min(y2)..=y1.max(y2)).collect();
        let x_range: Vec<i64> = (x1.min(x2)..=x1.max(x2)).collect();
        if x1 == x2 || y1 == y2 {
            for &y in y_range.iter() {
                for &x in x_range.iter() {
                    let c = 1 + *g1.get_default((y, x));
                    g1.set((y, x), c);
                    let c = 1 + *g2.get_default((y, x));
                    g2.set((y, x), c);
                }
            }
        } else {
            for (&y, x) in range(y1, y2, 1).iter().zip(range(x1, x2, 1)) {
                let c = 1 + *g2.get_default((y, x));
                g2.set((y, x), c);
            }
        }
    }
    let mut a = 0;
    for (_, c) in g1.to_sized().iter() {
        if c > 1 {
            a += 1;
        }
    }
    let mut b = 0;
    for (_, c) in g2.to_sized().iter() {
        if c > 1 {
            b += 1;
        }
    }
    format!("{} {}", a, b)
}

fn parse_point(p: (&str, &str)) -> Point {
    let (x, y) = p;
    (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
}
