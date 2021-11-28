use std::collections::HashMap;
use std::fmt::Display;
use std::string::ToString;

#[derive(Clone, Default, Debug)]
pub struct Grid<T> {
    g: HashMap<usize, T>,
    height: usize,
    width: usize,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            g: HashMap::<usize, T>::new(),
            height,
            width,
        }
    }

    fn key(&self, y: usize, x: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, y: usize, x: usize) -> &T {
        let key = self.key(y, x);
        self.g.get(&key).unwrap()
    }

    pub fn set(&mut self, y: usize, x: usize, value: T) {
        self.g.insert(self.key(y, x), value);
    }

    pub fn get_default(&mut self, y: usize, x: usize) -> &T {
        if !self.g.contains_key(&self.key(y, x)) {
            self.set(y, x, T::default());
        }
        self.get(y, x)
    }
}

impl<T> ToString for Grid<T>
where
    T: Default + Clone + Display,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.height * self.width);
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!("{}", self.get(y, x)));
            }
            s.push('\n');
        }
        s
    }
}
