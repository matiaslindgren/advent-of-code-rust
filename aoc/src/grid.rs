use crate::common::interpolate_2d_discrete;
use std::collections::HashMap;
use std::fmt::Display;
use std::string::ToString;

#[derive(Clone, Default, Debug)]
pub struct Grid<T> {
    g:          HashMap<i64, T>,
    pub height: usize,
    pub width:  usize,
}

impl<'a, T> Grid<T>
where
    T: Default + Clone + PartialEq,
{
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            g: HashMap::<i64, T>::new(),
            height,
            width,
        }
    }

    fn key(&self, y: i64, x: i64) -> i64 {
        y * (self.width as i64) + x
    }

    pub fn get(&self, y: i64, x: i64) -> Option<&T> {
        let key = self.key(y, x);
        self.g.get(&key)
    }

    pub fn set(&mut self, y: i64, x: i64, value: T) {
        self.g.insert(self.key(y, x), value);
    }

    pub fn get_default(&mut self, y: i64, x: i64) -> &T {
        if !self.g.contains_key(&self.key(y, x)) {
            self.set(y, x, T::default());
        }
        self.get(y, x).unwrap()
    }

    pub fn contains(&self, y: i64, x: i64, value: &T) -> bool {
        match self.get(y, x) {
            Some(v) => v == value,
            None => false,
        }
    }

    pub fn line_to(
        &self,
        p1: (i64, i64),
        p2: (i64, i64),
    ) -> Vec<(i64, i64, T)> {
        let line = interpolate_2d_discrete(p1, p2);
        line.iter()
            .map(|&(y, x)| {
                (y, x, self.get(y, x).unwrap_or(&T::default()).to_owned())
            })
            .collect()
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter::<T>::new(self)
    }
}

pub struct GridIter<'a, T> {
    g:     &'a Grid<T>,
    index: Vec<(i64, i64)>,
    i:     usize,
}

impl<'a, T> GridIter<'a, T> {
    fn new(g: &'a Grid<T>) -> Self {
        let mut index = Vec::<(i64, i64)>::new();
        for y in 0..g.height {
            for x in 0..g.width {
                index.push((y as i64, x as i64));
            }
        }
        Self { g, index, i: 0 }
    }
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Default + Clone + PartialEq,
{
    type Item = (i64, i64, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.index.len() {
            return None;
        }
        let (y, x) = self.index[self.i];
        self.i += 1;
        let item = match self.g.get(y, x) {
            Some(ch) => ch.clone(),
            None => T::default(),
        };
        Some((y, x, item))
    }
}

impl<T> ToString for Grid<T>
where
    T: Default + Clone + Display + PartialEq,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.height * self.width);
        for (_, x, ch) in self.iter() {
            s.push_str(&format!("{}", ch));
            if x == (self.width as i64) - 1 {
                s.push('\n');
            }
        }
        s
    }
}
