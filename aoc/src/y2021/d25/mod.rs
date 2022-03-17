use std::str;

pub fn main(input: &str) -> String {
    let grid = input.parse::<Grid>().expect("failed parsing grid");
    let a = find_a(&grid);
    let b = 0;
    format!("{} {}", a, b)
}

fn find_a(grid: &Grid) -> usize {
    let mut step_count = 0;
    let mut grid = grid.clone();
    loop {
        let next = grid.step(East).step(South);
        step_count += 1;
        if next == grid {
            break step_count;
        }
        grid = next;
    }
}

#[derive(Clone, PartialEq)]
struct Grid {
    slots: Vec<Slot>,
    width: usize,
}

impl Grid {
    fn step(&self, turn: Slot) -> Grid {
        let mut next_state = self.clone();
        for (src, &slot) in self.slots.iter().enumerate() {
            if slot == turn {
                let dst = self.next_pos(src, slot);
                if self.slots[dst] == Empty {
                    next_state.slots[dst] = self.slots[src];
                    next_state.slots[src] = Empty;
                }
            }
        }
        next_state
    }

    fn next_pos(&self, pos: usize, slot: Slot) -> usize {
        match slot {
            East => {
                let y = pos / self.width;
                let x = (pos + 1) % self.width;
                y * self.width + x
            }
            South => (pos + self.width) % self.slots.len(),
            Empty => panic!("empty slot has no next position"),
        }
    }
}

impl str::FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut slots = vec![];
        let mut width = 0;
        for line in s.lines() {
            width = line.len();
            for ch in line.chars() {
                let slot = match ch {
                    '>' => Ok(East),
                    'v' => Ok(South),
                    '.' => Ok(Empty),
                    x => Err(format!("unknown slot '{}'", x)),
                };
                slots.push(slot?);
            }
        }
        Ok(Self { slots, width })
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    East,
    South,
    Empty,
}
use Slot::{East, Empty, South};
