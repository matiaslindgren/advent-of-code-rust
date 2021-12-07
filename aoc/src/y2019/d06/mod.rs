use crate::common::pairs;
use crate::graph::Graph;

pub fn main(input: &str) -> String {
    let adjacencies = pairs::<String, String>(input, ")");
    let orbits = Graph::from(&adjacencies[..]);
    let a = find_a(&orbits);
    let b = find_b(&orbits);
    format!("{} {}", a, b)
}

fn find_a(orbits: &Graph) -> usize {
    orbits
        .adj
        .keys()
        .map(|k| orbits.count_children(k))
        .sum::<usize>()
}

fn find_b(orbits: &Graph) -> usize {
    orbits.adj.keys().fold(usize::MAX, |dist_orbital, obj| {
        let obj2you = orbits.distance(obj, "YOU");
        let obj2san = orbits.distance(obj, "SAN");
        let dist_you_san = obj2you.saturating_add(obj2san) - 2;
        dist_orbital.min(dist_you_san)
    })
}
