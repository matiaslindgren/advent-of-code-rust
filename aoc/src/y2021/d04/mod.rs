use crate::common;
use crate::grid::Grid;
use crate::grid::Point;

pub fn main(input: &str) -> String {
    let (numbers, boards) = input.split_once("\n\n").unwrap();
    let numbers = common::items::<i32>(numbers, ",");
    let boards: Vec<Grid<Mark>> = common::items::<String>(boards, "\n\n")
        .iter()
        .map(|s| parse_board(s))
        .collect();
    let scores = play(&numbers, &boards);
    let a = scores.first().unwrap();
    let b = scores.last().unwrap();
    format!("{} {}", a, b)
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Mark {
    num:     i32,
    checked: bool,
}

fn parse_board(board_str: &str) -> Grid<Mark> {
    let mut board = Grid::<Mark>::new(5, 5);
    for (y, line) in board_str.lines().enumerate() {
        let row = common::items::<i32>(line, " ");
        for (x, &num) in row.iter().enumerate() {
            let pos = (y as i64, x as i64);
            let mark = Mark {
                num,
                checked: false,
            };
            board.set(pos, mark);
        }
    }
    board
}

fn won(marks: &Grid<Mark>) -> bool {
    let checked = |row, col| match marks.get((row, col)) {
        Some(m) => m.checked,
        None => false,
    };
    let won_row = (0..5).any(|row| (0..5).all(|col| checked(row, col)));
    let won_col = (0..5).any(|col| (0..5).all(|row| checked(row, col)));
    won_row || won_col
}

fn unmarked_sum(board: &Grid<Mark>) -> i32 {
    board
        .iter()
        .filter(|(_, mark)| !mark.checked)
        .map(|(_, mark)| mark.num)
        .sum()
}

fn correct_marks(bingo_num: i32, board: &Grid<Mark>) -> Vec<Point> {
    board
        .iter()
        .filter(|(_, mark)| mark.num == bingo_num)
        .map(|(pos, _)| pos)
        .collect()
}

fn play(numbers: &[i32], boards: &[Grid<Mark>]) -> Vec<i32> {
    let mut boards = boards.to_vec();
    let mut scores = Vec::<i32>::new();
    for &bingo_num in numbers.iter() {
        for board in boards.iter_mut() {
            for &pos in correct_marks(bingo_num, board).iter() {
                let mut mark = board.get_mut(pos).unwrap();
                mark.checked = true;
            }
            if won(board) {
                scores.push(bingo_num * unmarked_sum(board));
            }
        }
        boards.retain(|b| !won(b));
    }
    scores
}
