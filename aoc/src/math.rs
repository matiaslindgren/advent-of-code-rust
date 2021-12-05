use crate::common::range;
use crate::grid::Point;

pub fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a.rem_euclid(b)),
    }
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

pub fn decimal_digits(x: &usize) -> Vec<usize> {
    let mut digits = Vec::<usize>::new();
    let mut x = *x;
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.reverse();
    digits
}

pub fn interpolate_2d_discrete(p1: Point, p2: Point) -> Vec<Point> {
    let (y1, x1) = p1;
    let (y2, x2) = p2;
    let dy = (y2 - y1).abs();
    let dx = (x2 - x1).abs();
    let d = gcd(dy, dx) as usize;
    if d == 0 {
        return vec![];
    }
    let dy = (dy as usize) / d;
    let dx = (dx as usize) / d;
    let y = match dy {
        0 => vec![y1; d + 1],
        _ => range(y1, y2, dy),
    };
    let x = match dx {
        0 => vec![x1; d + 1],
        _ => range(x1, x2, dx),
    };
    y.iter().cloned().zip(x).collect()
}
