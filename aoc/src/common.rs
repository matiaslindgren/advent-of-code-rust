use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub fn items<T>(input: &str, sep: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input.split(sep).flat_map(|x| str::parse(x)).collect()
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

pub fn range(a: i64, b: i64, step: usize) -> Vec<i64> {
    let v: Vec<i64> = if a < b {
        (a..=b).collect()
    } else {
        (b..=a).rev().collect()
    };
    v.iter().step_by(step).cloned().collect()
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

    pub fn most_common(&self) -> Vec<(T, usize)> {
        let mut f: Vec<(T, usize)> =
            self.g.keys().map(|k| (k.clone(), self.get(k))).collect();
        f.sort_unstable_by_key(|(k, _)| self.get(k));
        f.reverse();
        f
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vec3 {
    pub fn l1(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    pub fn signum(&self) -> Self {
        let x = self.x.signum();
        let y = self.y.signum();
        let z = self.z.signum();
        Self { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
