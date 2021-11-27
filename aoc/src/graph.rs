use std::collections::{hash_map::Keys, HashMap};

type Key = String;
type Value = String;
type Values = Vec<Value>;

pub struct Graph {
    g: HashMap<Key, Values>,
}

impl Graph {
    pub fn new() -> Self {
        let g = HashMap::<Key, Values>::new();
        Self { g }
    }

    pub fn keys(&self) -> Keys<Key, Values> {
        self.g.keys()
    }

    pub fn insert(&mut self, key: &str, val: &str) {
        if !self.g.contains_key(key) {
            self.g.insert(key.to_owned(), vec![]);
        }
        self.g.get_mut(key).unwrap().push(val.to_owned());
    }

    pub fn count_children(&self, k: &str) -> usize {
        match self.g.get(k) {
            Some(children) => children
                .iter()
                .fold(0, |n, c| n + 1 + self.count_children(c)),
            None => 0,
        }
    }

    pub fn distance(&self, src: &str, dst: &str) -> usize {
        if src == dst {
            return 0;
        }
        match self.g.get(src) {
            Some(children) => children.iter().fold(usize::MAX - 1, |dist, c| {
                dist.min(1 + self.distance(c, dst))
            }),
            None => usize::MAX - 1,
        }
    }
}

impl From<&Vec<(Key, Value)>> for Graph {
    fn from(adjacencies: &Vec<(Key, Value)>) -> Self {
        let mut g = Self::new();
        for (a, b) in adjacencies {
            g.insert(a, b);
        }
        g
    }
}
