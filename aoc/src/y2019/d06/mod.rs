use crate::{common, graph};

pub fn main(input: &str) -> String {
    let adjacencies = common::pairs::<String, String>(input, ")");
    let orbits = graph::Graph::from(&adjacencies);
    let a = find_a(&orbits);
    let b = find_b(&orbits);
    format!("{} {}", a, b)
}

fn find_a(orbits: &graph::Graph) -> usize {
    orbits.keys().fold(0, |n, k| n + orbits.count_children(k))
}

fn find_b(orbits: &graph::Graph) -> usize {
    orbits.keys().fold(usize::MAX, |dist_orbital, obj| {
        let obj2you = orbits.distance(obj, "YOU");
        let obj2san = orbits.distance(obj, "SAN");
        if obj2you < usize::MAX - 1 && obj2san < usize::MAX - 1 {
            dist_orbital.min(obj2you + obj2san - 2)
        } else {
            dist_orbital
        }
    })
}
