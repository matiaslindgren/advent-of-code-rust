use std::collections::HashMap;

type K = String;
type V = String;

#[derive(Clone, Default, Debug)]
pub struct Graph {
    pub adj: HashMap<K, Vec<V>>,
    edges:   HashMap<(K, K), i64>,
}

impl Graph
where
    V: Clone,
{
    pub fn new() -> Self {
        Self {
            adj:   HashMap::<K, Vec<V>>::new(),
            edges: HashMap::<(K, K), i64>::new(),
        }
    }

    pub fn insert(&mut self, k1: &str, k2: &str, w: i64) {
        if !self.adj.contains_key(k1) {
            self.adj.insert(k1.to_owned(), vec![]);
        }
        let k1_adj = self.adj.get_mut(k1).unwrap();
        k1_adj.push(k2.to_owned());
        let edge = (k1.to_owned(), k2.to_owned());
        self.edges.insert(edge, w);
    }

    pub fn count_children(&self, k: &str) -> usize {
        match self.adj.get(k) {
            Some(children) => children
                .iter()
                .map(|c| 1 + self.count_children(c))
                .sum::<usize>(),
            None => 0,
        }
    }

    pub fn distance(&self, src: &str, dst: &str) -> usize {
        if src == dst {
            return 0;
        }
        match self.adj.get(src) {
            Some(children) => children
                .iter()
                .map(|c| self.distance(c, dst).saturating_add(1))
                .min()
                .unwrap_or(usize::MAX),
            None => usize::MAX,
        }
    }
}

impl From<&[(K, V)]> for Graph {
    fn from(adjacencies: &[(K, V)]) -> Self {
        let mut g = Self::new();
        for (a, b) in adjacencies.iter() {
            g.insert(a, b, 0);
        }
        g
    }
}
