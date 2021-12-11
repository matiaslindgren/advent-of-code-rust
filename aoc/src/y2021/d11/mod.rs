use crate::grid::{grid2d, Grid, Point};

pub fn main(input: &str) -> String {
    let mut grid = grid2d::<i8>(input);
    let pos: Vec<Point> = grid.iter().map(|(p, _)| p).collect();
    let mut a = 0;
    let mut b = 0;
    loop {
        for &p in pos.iter() {
            *grid.get_mut(p).unwrap() += 1;
        }
        while grid.iter().any(|(_, o)| o > 9) {
            grid = flash_all(&grid);
        }
        let mut num_flashed = 0;
        for &p in pos.iter() {
            let o = grid.get_mut(p).unwrap();
            if *o == -1 {
                num_flashed += 1;
                *o = 0;
            }
        }
        a += num_flashed;
        b += 1;
        if num_flashed == pos.len() {
            break;
        }
    }
    format!("{} {}", a, b)
}

fn adjacent(pos: Point) -> Vec<Point> {
    let (y, x) = pos;
    vec![
        (y - 1, x - 1),
        (y - 1, x),
        (y - 1, x + 1),
        (y, x - 1),
        (y, x + 1),
        (y + 1, x - 1),
        (y + 1, x),
        (y + 1, x + 1),
    ]
}

fn flash_all(g_in: &Grid<i8>) -> Grid<i8> {
    let mut g_out = g_in.clone();
    let should_flash = g_in.iter().filter(|(_, o)| o > &9);
    for (p1, _) in should_flash {
        g_out.set(p1, -1);
        for &p2 in adjacent(p1).iter() {
            if let Some(o2) = g_out.get_mut(p2) {
                if *o2 != -1 {
                    *o2 += 1;
                }
            }
        }
    }
    g_out
}
