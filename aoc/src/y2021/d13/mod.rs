use crate::common::pairs;
use crate::grid::{Grid, Point};

pub fn main(input: &str) -> String {
    let (grid, folds) = parse(input);
    let a = find_a(&grid, &folds);
    let b = find_b(&grid, &folds);
    format!("{}\n{}", a, b)
}

fn parse(input: &str) -> (Grid<i8>, Vec<Point>) {
    let (points, folds) = input.split_once("\n\n").unwrap();
    let mut grid = Grid::<i8>::new(0, 0);
    for &(x, y) in pairs::<i64, i64>(points, ",").iter() {
        grid.set((y, x), 1);
    }
    let folds = folds
        .lines()
        .map(|line| {
            let (_, line) = line.rsplit_once(' ').unwrap();
            let (axis, pos) = line.split_once('=').unwrap();
            let pos = pos.parse::<i64>().unwrap();
            if axis == "y" {
                (pos, -1)
            } else {
                (-1, pos)
            }
        })
        .collect();
    (grid.to_sized(), folds)
}

fn find_a(grid: &Grid<i8>, fold_axes: &[Point]) -> usize {
    split_and_fold(grid, &fold_axes[0])
        .iter()
        .filter(|&(_, v)| v > 0)
        .count()
}

fn find_b(grid: &Grid<i8>, fold_axes: &[Point]) -> String {
    let mut grid = grid.clone();
    for axis in fold_axes.iter() {
        grid = split_and_fold(&grid, axis);
    }
    grid.to_sized().to_string().replace('0', " ")
}

fn split_and_fold(g: &Grid<i8>, axis: &Point) -> Grid<i8> {
    let &(y_split, x_split) = axis;
    let (h, w) = if y_split > 0 {
        (g.height / 2, g.width)
    } else {
        (g.height, g.width / 2)
    };
    let mut g1 = Grid::<i8>::new(h, w);
    let mut g2 = Grid::<i8>::new(h, w);
    for ((y, x), val) in g.iter() {
        if (x_split > 0 && x < x_split) || (y_split > 0 && y < y_split) {
            g1.set((y, x), val);
        }
        if (x_split > 0 && x > x_split) || (y_split > 0 && y > y_split) {
            let pos = if y_split > 0 {
                (y - h as i64 - 1, x)
            } else {
                (y, x - w as i64 - 1)
            };
            g2.set(pos, val);
        }
    }
    for (pos, val) in g2.flip(axis).iter() {
        if val > 0 {
            g1.set(pos, val);
        }
    }
    g1
}
