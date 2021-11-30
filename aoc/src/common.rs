use crate::grid;
use std::fmt::Debug;
use std::str::FromStr;

pub fn items<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.split(sep).map(|x| str::parse(x).unwrap()).collect()
}

pub fn pairs<A, B>(input: &str, line_sep: &str) -> Vec<(A, B)>
where
    A: FromStr,
    B: FromStr,
    <A as FromStr>::Err: Debug,
    <B as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(line_sep);
            let a = parts.next();
            let b = parts.next();
            (a.unwrap(), b.unwrap())
        })
        .map(|(a, b)| {
            let a = str::parse::<A>(a);
            let b = str::parse::<B>(b);
            (a.unwrap(), b.unwrap())
        })
        .collect()
}

pub fn directions<Label, Distance>(
    input: &str,
    sep: &str,
    label_len: usize,
) -> Vec<(Label, Distance)>
where
    Label: FromStr,
    Distance: FromStr,
{
    let labels = input
        .split(sep)
        .filter_map(|x| str::parse::<Label>(&x[..label_len]).ok());
    let distances = input
        .split(sep)
        .filter_map(|x| str::parse::<Distance>(&x[label_len..]).ok());
    labels.zip(distances).collect()
}

pub fn decimal_digits(x: &usize) -> Vec<usize> {
    let mut digits = Vec::<usize>::new();
    let mut x = *x;
    while x > 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits.reverse();
    digits
}

pub fn linspace(a: i64, b: i64, n: usize) -> Vec<i64> {
    let a = a as f64;
    let b = b as f64;
    let delta = (b - a) / ((n - 1) as f64);
    (0..n)
        .map(|i| (a + delta * (i as f64)).floor() as i64)
        .collect()
}

pub fn maze(input: &str) -> grid::Grid<char> {
    let lines: Vec<String> = items::<String>(input, "\n");
    let (h, w) = (lines.len(), lines[0].len());
    let mut g = grid::Grid::<char>::new(h, w);
    for (y, line) in lines.iter().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            g.set(y as i64, x as i64, ch);
        }
    }
    g
}
