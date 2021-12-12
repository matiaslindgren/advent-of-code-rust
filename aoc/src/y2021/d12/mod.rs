use crate::common::pairs;
use std::collections::HashMap;

pub fn main(input: &str) -> String {
    let adj = parse_adjacencies(input);
    let a = find(&adj, true);
    let b = find(&adj, false);
    format!("{} {}", a, b)
}

fn parse_adjacencies(input: &str) -> Adj {
    let insert = |adj: &mut Adj, v1: &str, v2: &str| {
        if !adj.contains_key(v1) {
            adj.insert(v1.to_string(), vec![]);
        }
        adj.get_mut(v1).unwrap().push(v2.to_string());
    };
    let mut adjacent = Adj::new();
    for (v1, v2) in pairs::<String, String>(input, "-").iter() {
        insert(&mut adjacent, v1, v2);
        insert(&mut adjacent, v2, v1);
    }
    adjacent
}

type Adj = HashMap<String, Vec<String>>;

fn find(adj: &Adj, is_part_a: bool) -> usize {
    let mut paths = Vec::<Vec<String>>::new();
    find_paths(
        "start",
        adj,
        &Counter::new(),
        &mut vec![],
        &mut paths,
        is_part_a,
    );
    paths.len()
}

fn find_paths(
    cave: &str,
    adj: &Adj,
    visit_count: &Counter,
    path: &mut Vec<String>,
    paths: &mut Vec<Vec<String>>,
    is_part_a: bool,
) {
    let num_visits = *visit_count.get(cave).unwrap_or(&0);
    if num_visits > 0 {
        if is_part_a || !is_small_cave(cave) {
            return;
        }
        if num_visits > 1 || num_small_with_multiple_visits(visit_count) > 0 {
            return;
        }
    }
    path.push(cave.to_string());
    if cave == "end" {
        paths.push(path.clone());
        return;
    }
    let mut visit_count = visit_count.clone();
    if !is_big_cave(cave) {
        inc(&mut visit_count, cave);
    }
    for next_cave in adj.get(cave).unwrap_or(&vec![]).iter() {
        find_paths(
            next_cave,
            adj,
            &visit_count.clone(),
            &mut path.clone(),
            paths,
            is_part_a,
        );
    }
}

fn num_small_with_multiple_visits(c: &Counter) -> usize {
    c.iter()
        .filter(|(k, v)| is_small_cave(k) && **v > 1)
        .count()
}

fn is_small_cave(s: &str) -> bool {
    s != "start" && s != "end" && !is_big_cave(s)
}

fn is_big_cave(s: &str) -> bool {
    s.chars().all(|ch| ch.is_ascii_uppercase())
}

type Counter = HashMap<String, usize>;

fn inc(c: &mut Counter, k: &str) {
    if !c.contains_key(k) {
        c.insert(k.to_string(), 0);
    }
    *c.get_mut(k).unwrap() += 1;
}
