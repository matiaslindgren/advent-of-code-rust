use crate::grid;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
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

pub fn range(a: i64, b: i64, step: usize) -> Vec<i64> {
    let v: Vec<i64> = if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    };
    v.iter().step_by(step).cloned().collect()
}

fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a.rem_euclid(b)),
    }
}

pub fn interpolate_2d_discrete(
    p1: (i64, i64),
    p2: (i64, i64),
) -> Vec<(i64, i64)> {
    let (y1, x1) = p1;
    let (y2, x2) = p2;
    let dy = (y2 - y1).abs();
    let dx = (x2 - x1).abs();
    let d = gcd(dy, dx) as usize;
    if d == 0 {
        return vec![];
    }
    let dy = (dy as usize) / d;
    let dx = (dx as usize) / d;
    let y = match dy {
        0 => vec![y1; d + 1],
        _ => range(y1, y2, dy),
    };
    let x = match dx {
        0 => vec![x1; d + 1],
        _ => range(x1, x2, dx),
    };
    y.iter().cloned().zip(x).collect()
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

#[derive(Clone, Default, Debug)]
pub struct Counter<T> {
    g: HashMap<T, usize>,
}

impl<T> Counter<T>
where
    T: Default + Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            g: HashMap::<T, usize>::new(),
        }
    }

    pub fn get(&self, k: &T) -> usize {
        match self.g.get(k) {
            Some(c) => *c,
            None => 0,
        }
    }

    pub fn inc(&mut self, k: &T) -> usize {
        if !self.g.contains_key(k) {
            self.g.insert(k.clone(), 0);
        }
        let c = self.g.get_mut(k).unwrap();
        *c += 1;
        *c
    }
}
