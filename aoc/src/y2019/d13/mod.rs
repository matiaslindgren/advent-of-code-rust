use crate::grid::Grid;
use crate::y2019::intcode::IntCode;

pub fn main(input: &str) -> String {
    let a = find_a(input);
    let b = find_b(input);
    format!("{} {}", a, b)
}

fn find_a(input: &str) -> usize {
    let mut ic = IntCode::new(input);
    let mut g = Grid::<i64>::new(0, 0);
    loop {
        let x = ic.run();
        let y = ic.run();
        let t = ic.run();
        if [x, y, t].iter().any(|o| o.is_none()) {
            break;
        }
        let pos = (y.unwrap(), x.unwrap());
        let tile = t.unwrap();
        g.set(pos, tile);
    }
    g.g.values().filter(|v| **v == 2).count()
}

fn find_b(input: &str) -> i64 {
    let mut ic = IntCode::new(input);
    ic.store(0, 2);
    let mut g = Grid::<i64>::new(0, 0);
    let mut score = 0;
    let mut paddle = None;
    loop {
        let x = ic.run();
        let y = ic.run();
        let t = ic.run();
        if [x, y, t].iter().any(|o| o.is_none()) {
            break score;
        }
        let pos = (y.unwrap(), x.unwrap());
        if pos == (0, -1) {
            score = t.unwrap();
            continue;
        }
        let tile = t.unwrap();
        g.set(pos, tile);
        if tile == 3 {
            paddle = Some(pos);
        } else if tile == 4 {
            let (_, x_ball) = pos;
            let paddle_dx = match paddle {
                Some((_, x_paddle)) => (x_ball - x_paddle).signum(),
                _ => 0,
            };
            ic.push_input(paddle_dx);
        }
    }
}
