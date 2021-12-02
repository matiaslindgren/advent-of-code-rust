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
    ) -> Vec<((i64, i64), T)> {
        let line = interpolate_2d_discrete(p1, p2);
        line.iter()
            .map(|&(y, x)| {
                ((y, x), self.get(y, x).unwrap_or(&T::default()).to_owned())
            })
            .collect()
    }

    pub fn iter(&self) -> GridIter<T> {
        GridIter::<T>::new(self)
    }
}

pub struct GridIter<'a, T> {
    g: &'a Grid<T>,
    y: i64,
    x: i64,
}

impl<'a, T> GridIter<'a, T> {
    fn new(g: &'a Grid<T>) -> Self {
        Self { g, y: 0, x: -1 }
    }

    fn next_index(&mut self) -> Option<(i64, i64)> {
        self.x += 1;
        if self.x as usize == self.g.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y as usize == self.g.height {
            None
        } else {
            Some((self.y, self.x))
        }
    }
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Default + Clone + PartialEq,
{
    type Item = ((i64, i64), T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_index() {
            Some((y, x)) => {
                let item = match self.g.get(y, x) {
                    Some(ch) => ch.clone(),
                    None => T::default(),
                };
                Some(((y, x), item))
            }
            None => None,
        }
    }
}

impl<T> ToString for Grid<T>
where
    T: Default + Clone + Display + PartialEq,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.height * self.width);
        for ((y, x), ch) in self.iter() {
            if y != 0 && x == 0 {
                s.push('\n');
            }
            s.push_str(&format!("{}", ch));
        }
        s
    }
}
