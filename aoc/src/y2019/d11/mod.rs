use crate::grid::Grid;
use crate::y2019::intcode::IntCode;

pub fn main(input: &str) -> String {
    let a = find_a(input);
    let b = find_b(input);
    format!("{}\n{}", a, b)
}

fn find_a(input: &str) -> usize {
    let mut grid = Grid::<i64>::new(0, 0);
    run_robot(input, &mut grid);
    grid.count()
}

fn find_b(input: &str) -> String {
    let mut grid = Grid::<i64>::new(0, 0);
    grid.set((0, 0), 1);
    run_robot(input, &mut grid);
    grid.to_sized()
        .flip_y()
        .to_string()
        .replace("0", " ")
        .replace("1", "#")
}

enum Direction {
    U,
    R,
    D,
    L,
}

struct Robot {
    y:   i64,
    x:   i64,
    dir: Direction,
}

impl Robot {
    fn new() -> Self {
        Self {
            y:   0,
            x:   0,
            dir: Direction::U,
        }
    }

    fn turn_left(&mut self) {
        use Direction::{D, L, R, U};
        self.dir = match self.dir {
            U => L,
            R => U,
            D => R,
            L => D,
        };
    }

    fn turn_right(&mut self) {
        use Direction::{D, L, R, U};
        self.dir = match self.dir {
            L => U,
            U => R,
            R => D,
            D => L,
        };
    }

    fn step(&mut self) {
        use Direction::{D, L, R, U};
        match self.dir {
            U => self.y += 1,
            R => self.x += 1,
            D => self.y -= 1,
            L => self.x -= 1,
        };
    }

    fn pos(&self) -> (i64, i64) {
        (self.y, self.x)
    }
}

fn run_robot(program: &str, grid: &mut Grid<i64>) {
    let mut ic = IntCode::new(program);
    let mut robot = Robot::new();
    loop {
        let current_color = *grid.get_default(robot.pos());
        ic.push_input(current_color);
        let new_color = ic.run();
        match new_color {
            Some(color) => grid.set(robot.pos(), color),
            None => break,
        }
        let turn = ic.run().unwrap();
        match turn {
            0 => robot.turn_left(),
            1 => robot.turn_right(),
            d => panic!("unknown direction {}", d),
        };
        robot.step();
    }
}
