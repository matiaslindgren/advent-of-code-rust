pub fn main(input: &str) -> String {
    let a = find_a(input);
    let b = find_b(input);
    format!("{} {}", a, b)
}

fn find_a(input: &str) -> i32 {
    let result = input.lines().map(str::to_owned).reduce(add).unwrap();
    magnitude(&result)
}

fn find_b(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(str::to_owned).collect();
    let mut b = 0;
    for (i, x) in lines.iter().enumerate() {
        for (j, y) in lines.iter().enumerate() {
            if i != j {
                let added = add(x.to_string(), y.to_string());
                b = b.max(magnitude(&added));
            }
        }
    }
    b
}

fn add(s1: String, s2: String) -> String {
    let mut s = format!("[{},{}]", s1, s2);
    loop {
        if let Some(explode_begin) = find_exploding(&s) {
            s = explode(&s, explode_begin);
            continue;
        }
        if let Some(split_begin) = find_splitting(&s) {
            s = split(&s, split_begin);
            continue;
        }
        return s;
    }
}

fn find_exploding(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    let mut depth = 0;
    for (i, w) in chars.windows(2).enumerate() {
        if w[0] == '[' {
            depth += 1;
            if depth > 4 && w[1].is_numeric() {
                return Some(i);
            }
        } else if w[0] == ']' {
            depth -= 1;
        }
    }
    None
}

fn find_splitting(s: &str) -> Option<usize> {
    let chars: Vec<char> = s.chars().collect();
    for (i, w) in chars.windows(2).enumerate() {
        if w[0].is_numeric() && w[1].is_numeric() {
            return Some(i);
        }
    }
    None
}

fn explode(s: &str, pair_begin: usize) -> String {
    let pair_end = pair_begin + (&s[pair_begin..]).find(']').unwrap() + 1;
    let left = if let Some(l) = find_left(s, pair_begin) {
        let left = parse_number(s, l);
        let x = parse_number(s, pair_begin + 1);
        let res = format!("{}", left + x);
        let after_left = find_next_non_numeric(s, l);
        [&s[..l], &res, &s[after_left..pair_begin]].join("")
    } else {
        (&s[..pair_begin]).to_string()
    };
    let right = if let Some(r) = find_right(s, pair_end) {
        let y_pos = 1 + &s[..pair_end - 1].rfind(not_numeric).unwrap();
        let y = parse_number(s, y_pos);
        let right = parse_number(s, r);
        let res = format!("{}", y + right);
        let after_right = find_next_non_numeric(s, r);
        [&s[pair_end..r], &res, &s[after_right..]].join("")
    } else {
        (&s[pair_end..]).to_string()
    };
    [&left, "0", &right].join("")
}

fn find_next_non_numeric(s: &str, i: usize) -> usize {
    if let Some(end) = (&s[i..]).find(not_numeric) {
        i + end
    } else {
        s.len()
    }
}

fn not_numeric(c: char) -> bool {
    !c.is_numeric()
}

fn find_left(s: &str, pair_begin: usize) -> Option<usize> {
    let end = 1 + (&s[..pair_begin]).rfind(char::is_numeric)?;
    let begin = 1 + (&s[..end]).rfind(not_numeric)?;
    Some(begin)
}

fn find_right(s: &str, pair_end: usize) -> Option<usize> {
    let begin = (&s[pair_end..]).find(char::is_numeric)?;
    Some(pair_end + begin)
}

fn split(s: &str, num_begin: usize) -> String {
    let num_end = find_next_non_numeric(s, num_begin);
    let n = parse_number(s, num_begin);
    let x = n / 2;
    let y = (n + 1) / 2;
    [&s[..num_begin], &format!("[{},{}]", x, y), &s[num_end..]].join("")
}

fn parse_number(s: &str, i: usize) -> i32 {
    let end = find_next_non_numeric(s, i);
    (&s[i..end]).parse::<i32>().unwrap()
}

fn magnitude(s: &str) -> i32 {
    if s.chars().all(char::is_numeric) {
        parse_number(s, 0) as i32
    } else {
        let (left, right) = split_root_pair(s).unwrap();
        3 * magnitude(left) + 2 * magnitude(right)
    }
}

fn split_root_pair(s: &str) -> Option<(&str, &str)> {
    let s = &s[1..s.len() - 1];
    let mut depth = 0;
    for (i, ch) in s.chars().enumerate() {
        depth += (ch == '[') as i32;
        depth -= (ch == ']') as i32;
        if depth == 0 && ch == ',' {
            let (l, r) = s.split_at(i);
            return Some((l, &r[1..]));
        }
    }
    None
}
