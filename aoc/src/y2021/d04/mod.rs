use std::str;

pub fn main(input: &str) -> String {
    let (numbers, boards) = parse_input(input);
    let scores = play(&numbers, &boards);
    let a = scores.first().unwrap();
    let b = scores.last().unwrap();
    format!("{} {}", a, b)
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let (numbers, boards) = input
        .split_once("\n\n")
        .expect("expected input split by sections");
    let numbers = numbers
        .split(',')
        .map(|x| x.parse::<i32>().expect("failed parsing number"))
        .collect();
    let boards = boards
        .split("\n\n")
        .map(|s| s.parse::<Board>().expect("failed parsing board"))
        .collect();
    (numbers, boards)
}

#[derive(Clone)]
struct Board {
    marks: Vec<Mark>,
    width: usize,
}

#[derive(Clone)]
struct Mark {
    num:     i32,
    checked: bool,
}

impl str::FromStr for Board {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let marks = s
            .split_ascii_whitespace()
            .map(|num| num.parse::<i32>().expect("failed parsing number"))
            .map(|num| Mark {
                num,
                checked: false,
            })
            .collect();
        Ok(Self { marks, width: 5 })
    }
}

impl Board {
    fn is_marked(&self, y: usize, x: usize) -> bool {
        self.marks[y * self.width + x].checked
    }

    fn won(&self) -> bool {
        let w = self.width;
        let won_row = (0..w).any(|y| (0..w).all(|x| self.is_marked(y, x)));
        let won_col = (0..w).any(|x| (0..w).all(|y| self.is_marked(y, x)));
        won_row || won_col
    }

    fn sum_of_unmarked(&self) -> i32 {
        self.marks
            .iter()
            .filter_map(
                |mark| if !mark.checked { Some(mark.num) } else { None },
            )
            .sum()
    }

    fn update_marks(&mut self, bingo_num: i32) {
        for mark in self.marks.iter_mut() {
            if mark.num == bingo_num {
                mark.checked = true;
            }
        }
    }
}

fn play(numbers: &[i32], boards: &[Board]) -> Vec<i32> {
    let mut boards = boards.to_vec();
    let mut scores = vec![];
    for &bingo_num in numbers.iter() {
        for board in boards.iter_mut() {
            board.update_marks(bingo_num);
            if board.won() {
                scores.push(bingo_num * board.sum_of_unmarked());
            }
        }
        boards.retain(|b| !b.won());
    }
    scores
}
