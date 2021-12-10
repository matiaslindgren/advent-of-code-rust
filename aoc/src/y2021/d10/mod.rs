pub fn main(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let a = find_a(&lines);
    let b = find_b(&lines);
    format!("{} {}", a, b)
}

fn find_a(lines: &[&str]) -> i64 {
    let get_score = |c| match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    };
    lines
        .iter()
        .cloned()
        .filter_map(|l| {
            let (_, invalid) = parse_parens(l);
            invalid.get(0).cloned()
        })
        .map(get_score)
        .sum()
}

fn find_b(lines: &[&str]) -> i64 {
    let get_score = |c| match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    };
    let mut scores: Vec<i64> = lines
        .iter()
        .cloned()
        .map(parse_parens)
        .filter(|(_, invalid)| invalid.is_empty())
        .map(|(opened, _)| {
            opened
                .iter()
                .rev()
                .map(|&ch| get_score(ch))
                .fold(0, |total, score| 5 * total + score)
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn parse_parens(line: &str) -> (Vec<char>, Vec<char>) {
    let get_closing = |open| match open {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => ' ',
    };
    let is_valid_pair = |open: Option<char>, close| match open {
        Some(o) => get_closing(o) == close,
        None => false,
    };
    let mut opened = Vec::<char>::new();
    let mut invalid = Vec::<char>::new();
    for paren in line.chars() {
        if get_closing(paren) != ' ' {
            opened.push(paren);
        } else if !is_valid_pair(opened.pop(), paren) {
            invalid.push(paren);
        }
    }
    (opened, invalid)
}
