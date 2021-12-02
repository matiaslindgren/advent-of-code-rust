use crate::common::interpolate_2d_discrete;
use std::collections::HashMap;
use std::fmt::Display;
use std::string::ToString;

pub type Point = (i64, i64);

#[derive(Clone, Default, Debug)]
pub struct Grid<T> {
    g:          HashMap<Point, T>,
    pub height: usize,
    pub width:  usize,
}

impl<'a, T> Grid<T>
where
    T: Default + Clone + PartialEq,
{
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            g: HashMap::<Point, T>::new(),
            height,
            width,
        }
    }

    pub fn get(&self, pos: Point) -> Option<&T> {
        self.g.get(&pos)
    }

    pub fn set(&mut self, pos: Point, value: T) {
        self.g.insert(pos, value);
    }

    pub fn get_default(&mut self, pos: Point) -> &T {
        if !self.g.contains_key(&pos) {
            self.set(pos, T::default());
        }
        self.get(pos).unwrap()
    }

    pub fn contains(&self, pos: Point, value: &T) -> bool {
        match self.get(pos) {
            Some(v) => v == value,
            None => false,
        }
    }

    pub fn count(&self) -> usize {
        self.g.keys().count()
    }

    pub fn line_to(&self, p1: Point, p2: Point) -> Vec<(Point, T)> {
        let line = interpolate_2d_discrete(p1, p2);
        line.iter()
            .map(|&pos| {
                (pos, self.get(pos).unwrap_or(&T::default()).to_owned())
            })
            .collect()
    }

    pub fn to_sized(&self) -> Grid<T> {
        let (ymin, xmin, ymax, xmax) = self.g.keys().fold(
            (i64::MAX, i64::MAX, i64::MIN, i64::MIN),
            |(ymin, xmin, ymax, xmax), &(y, x)| {
                (ymin.min(y), xmin.min(x), ymax.max(y), xmax.max(x))
            },
        );
        let width = (xmin - xmax).abs() as usize;
        let height = (ymin - ymax).abs() as usize;
        let mut g = Grid::<T>::new(height + 1, width + 1);
        for ((y, x), v) in self.g.iter() {
            g.set((y - ymin, x - xmin), v.clone());
        }
        g
    }

    pub fn flip_y(&self) -> Grid<T> {
        let mut g = Grid::<T>::new(self.height, self.width);
        for ((y, x), v) in self.iter() {
            let new_y = self.height as i64 - y - 1;
            g.set((new_y, x), v.clone());
        }
        g
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

    fn next_index(&mut self) -> Option<Point> {
        self.x += 1;
        if self.x as usize == self.g.width {
            self.x = 0;
            self.y += 1;
        }
        if self.y as usize == self.g.height || self.g.height == 0 {
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
    type Item = (Point, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_index() {
            Some(pos) => {
                let item = match self.g.get(pos) {
                    Some(ch) => ch.clone(),
                    None => T::default(),
                };
                Some((pos, item))
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
