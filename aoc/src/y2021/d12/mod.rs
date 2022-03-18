use std::collections::HashMap;
use std::str;

pub fn main<'a>(input: &'a str) -> String {
    let adj = Adjacencies::from(input);
    let a = adj.find(true);
    let b = adj.find(false);
    format!("{} {}", a, b)
}

#[derive(Clone, Default)]
struct Adjacencies<'a> {
    adj: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> From<&'a str> for Adjacencies<'a> {
    fn from(s: &'a str) -> Self {
        let pairs = s.lines().map(|l| {
            l.split_once('-').expect("expected line separated by '-'")
        });
        let mut adjacent = Self::default();
        for (v1, v2) in pairs {
            adjacent.insert(v1, v2);
            adjacent.insert(v2, v1);
        }
        adjacent
    }
}

impl<'a> Adjacencies<'a> {
    fn insert(&mut self, v1: &'a str, v2: &'a str) {
        if let Some(a) = self.adj.get_mut(v1) {
            a.push(v2);
        } else {
            self.adj.insert(v1, vec![v2]);
        }
    }

    fn find(&self, is_part_a: bool) -> usize {
        let mut paths = vec![];
        self.find_paths(
            "start",
            &Counter::default(),
            &mut vec![],
            &mut paths,
            is_part_a,
        );
        paths.len()
    }

    fn find_paths(
        &self,
        cave: &'a str,
        visit_count: &Counter,
        path: &mut Vec<&'a str>,
        paths: &mut Vec<Vec<&'a str>>,
        is_part_a: bool,
    ) {
        if let Some(&num_visits) = visit_count.c.get(cave) {
            if is_part_a || !is_small_cave(cave) {
                return;
            }
            if num_visits > 1 || visit_count.small_visited_multiple_times() {
                return;
            }
        }
        path.push(cave);
        if cave == "end" {
            paths.push(path.clone());
            return;
        }
        let mut visit_count = visit_count.clone();
        if !is_big_cave(cave) {
            visit_count.inc(cave);
        }
        for next_cave in self.adj.get(cave).unwrap_or(&vec![]).iter() {
            self.find_paths(
                next_cave,
                &visit_count.clone(),
                &mut path.clone(),
                paths,
                is_part_a,
            );
        }
    }
}

fn is_small_cave(s: &str) -> bool {
    s != "start" && s != "end" && !is_big_cave(s)
}

fn is_big_cave(s: &str) -> bool {
    s.chars().all(|ch| ch.is_ascii_uppercase())
}

#[derive(Clone, Default)]
struct Counter<'a> {
    c: HashMap<&'a str, usize>,
}

impl<'a> Counter<'a> {
    fn inc(&mut self, k: &'a str) {
        if let Some(n) = self.c.get_mut(k) {
            *n += 1;
        } else {
            self.c.insert(k, 1);
        }
    }

    fn small_visited_multiple_times(&self) -> bool {
        self.c.iter().any(|(k, &v)| is_small_cave(k) && v > 1)
    }
}
