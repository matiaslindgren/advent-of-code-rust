use crate::common;
use crate::grid::Grid;

pub fn main(input: &str) -> String {
    let (input, boards) = input.split_once("\n\n").unwrap();
    let input = common::items::<i32>(input, ",");
    let boards = common::items::<String>(boards, "\n\n");
    let boards: Vec<(Grid<i32>, Grid<bool>)> = boards
        .iter()
        .map(|b| {
            let mut g = Grid::<i32>::new(5, 5);
            for (y, line) in b.trim().lines().enumerate() {
                for (x, v) in
                    common::items::<i32>(line.trim(), " ").iter().enumerate()
                {
                    g.set((y as i64, x as i64), *v)
                }
            }
            let m = Grid::<bool>::new(5, 5);
            (g, m)
        })
        .collect();
    let a = find_a(&input, &boards);
    let b = find_b(&input, &boards);
    format!("{} {}", a, b)
}

fn won(marks: &Grid<bool>) -> bool {
    let is_marked = |y, x| match marks.get((y, x)) {
        Some(m) => *m,
        None => false,
    };
    for y in 0..(marks.height as i64) {
        let mut w = true;
        for x in 0..(marks.width as i64) {
            w = w && is_marked(y, x);
        }
        if w {
            return true;
        }
    }
    for x in 0..(marks.height as i64) {
        let mut w = true;
        for y in 0..(marks.width as i64) {
            w = w && is_marked(y, x);
        }
        if w {
            return true;
        }
    }
    false
}

fn find_a(input: &[i32], boards: &[(Grid<i32>, Grid<bool>)]) -> i32 {
    let mut boards = boards.to_vec();
    let mut winner_idx = None;
    let mut winner_num = -1;
    'outer: for &i in input.iter() {
        for (w, (b, m)) in boards.iter_mut().enumerate() {
            for (pos, j) in b.iter() {
                if i == j {
                    m.set(pos, true);
                }
            }
            if winner_idx.is_none() && won(m) {
                winner_idx = Some(w);
                winner_num = i;
                break 'outer;
            }
        }
    }
    let (b, m) = &boards[winner_idx.unwrap()];
    let mut unmarked_sum = 0;
    for (pos, j) in b.iter() {
        unmarked_sum += match m.get(pos) {
            Some(x) => {
                if *x {
                    0
                } else {
                    j
                }
            }
            None => j,
        };
    }
    winner_num * unmarked_sum
}

fn find_b(input: &[i32], boards: &[(Grid<i32>, Grid<bool>)]) -> i32 {
    let mut boards = boards.to_vec();
    let mut has_won = vec![false; boards.len()];
    let mut winner_idx = None;
    let mut winner_num = -1;
    'outer: for &i in input.iter() {
        for (w, (b, m)) in boards.iter_mut().enumerate() {
            for (pos, j) in b.iter() {
                if i == j {
                    m.set(pos, true);
                }
            }
            if !has_won[w] && won(m) {
                winner_idx = Some(w);
                winner_num = i;
                has_won[w] = true;
                if has_won.iter().all(|x| *x) {
                    break 'outer;
                }
            }
        }
    }
    let (b, m) = &boards[winner_idx.unwrap()];
    let mut unmarked_sum = 0;
    for (pos, j) in b.iter() {
        unmarked_sum += match m.get(pos) {
            Some(x) => {
                if *x {
                    0
                } else {
                    j
                }
            }
            None => j,
        };
    }
    winner_num * unmarked_sum
}
