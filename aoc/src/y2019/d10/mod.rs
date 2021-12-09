use crate::grid::{grid2d, Grid, Point};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn main(input: &str) -> String {
    let m = grid2d::<char>(input);
    let (pos, a) = find_a(&m);
    let b = find_b(&m, pos);
    format!("{} {}", a, b)
}

fn view_blocked(m: &Grid<char>, p1: Point, p2: Point) -> bool {
    m.line_to(p1, p2)
        .iter()
        .any(|&(p, ch)| p != p1 && p != p2 && ch == '#')
}

fn find_a(m: &Grid<char>) -> (Point, usize) {
    let mut best_pos = (0, 0);
    let mut best_count = 0;
    for (p1, _) in m.iter().filter(|&(_, ch)| ch == '#') {
        let visible = m
            .iter()
            .filter(|&(p2, ch)| {
                p1 != p2 && ch == '#' && !view_blocked(m, p1, p2)
            })
            .count();
        if visible > best_count {
            best_pos = p1;
            best_count = visible;
        }
    }
    (best_pos, best_count)
}

#[derive(Debug)]
struct Asteroid {
    y:      i64,
    x:      i64,
    dist:   i64,
    angle:  f64,
    exists: bool,
}

fn find_b(m: &Grid<char>, laser_pos: Point) -> i64 {
    let mut asteroids: Vec<Asteroid> = m
        .iter()
        .filter(|&(pos, ch)| pos != laser_pos && ch == '#')
        .map(|((y, x), _)| {
            let (y_l, x_l) = laser_pos;
            let dy = y_l - y;
            let dx = x_l - x;
            let dist = dy * dy + dx * dx;
            let mut angle = (dy as f64).atan2(dx as f64);
            angle -= FRAC_PI_2;
            if angle < 0.0 {
                angle += TAU;
            }
            Asteroid {
                y,
                x,
                dist,
                angle,
                exists: true,
            }
        })
        .collect();
    asteroids.sort_unstable_by(|a1, a2| {
        a1.angle
            .partial_cmp(&a2.angle)
            .unwrap_or_else(|| a1.dist.cmp(&a2.dist))
    });
    let mut vaporized = 0;
    loop {
        let mut prev_angle = None;
        for a in asteroids.iter_mut().filter(|a| a.exists) {
            if prev_angle.is_none() || a.angle != prev_angle.unwrap() {
                a.exists = false;
                vaporized += 1;
                if vaporized == 200 {
                    return 100 * a.x + a.y;
                }
            }
            prev_angle = Some(a.angle);
        }
    }
}
