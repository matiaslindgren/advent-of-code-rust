use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

pub fn main(input: &str) -> String {
    let scanners = parse_scanners(input);
    let scanners = recenter_scanners(&scanners);
    let a = find_a(&scanners);
    let b = find_b(&scanners);
    format!("{} {}", a, b)
}

fn parse_scanners(input: &str) -> Vec<Scanner> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(id, section)| {
            let (_, beacons) = section.split_once("\n").unwrap();
            let beacons = beacons
                .lines()
                .map(|line| {
                    let xyz: Vec<i32> = line
                        .split(',')
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
                    Vec3 {
                        x: xyz[0],
                        y: xyz[1],
                        z: xyz[2],
                    }
                })
                .collect();
            Scanner {
                id,
                center: Vec3::default(),
                beacons,
            }
        })
        .collect()
}

fn recenter_scanners(scanners: &[Scanner]) -> Vec<Scanner> {
    let mut scanners = scanners.to_vec();
    let mut locked = vec![scanners.remove(0)];
    while !scanners.is_empty() {
        let next_overlapping = locked
            .iter()
            .find_map(|s1| {
                scanners.iter().find_map(|s2| {
                    if let Some(mut s2) = s1.rotate_until_overlap(s2) {
                        s2.center_on(s1);
                        Some(s2)
                    } else {
                        None
                    }
                })
            })
            .unwrap();
        scanners.retain(|s| s.id != next_overlapping.id);
        locked.push(next_overlapping);
    }
    locked
}

fn find_a(scanners: &[Scanner]) -> usize {
    let beacons: HashSet<Vec3> = scanners
        .iter()
        .map(|s| s.beacons.iter())
        .flatten()
        .cloned()
        .collect();
    beacons.len()
}

fn find_b(scanners: &[Scanner]) -> usize {
    scanners
        .iter()
        .map(move |s1| {
            scanners.iter().map(move |s2| {
                let d = s2.center - s1.center;
                (d.x.abs() + d.y.abs() + d.z.abs()) as usize
            })
        })
        .flatten()
        .max()
        .unwrap()
}

#[derive(Debug, Default, Clone)]
struct Scanner {
    id:      usize,
    center:  Vec3,
    beacons: Vec<Vec3>,
}

impl Scanner {
    fn find_common(&self, common: &Common) -> Common {
        let mut common = common.clone();
        for (&b1, &b2) in self.beacon_pairs() {
            let delta = b1 - b2;
            if let Some((start, end)) = common.get_mut(&delta) {
                (*start).push(b1);
                (*end).push(b2);
            } else {
                common.insert(delta, (vec![b1], vec![b2]));
            }
        }
        common
    }

    fn rotate_until_overlap(&self, other: &Self) -> Option<Self> {
        let common = self.find_common(&Common::default());
        let count_overlapping = |candidate: &Self| -> usize {
            candidate
                .beacon_pairs()
                .filter(|(&b1, &b2)| common.contains_key(&(b1 - b2)))
                .count()
        };
        other
            .all_rotations()
            .into_iter()
            .find(|candidate| count_overlapping(&candidate) == 132)
    }

    fn all_rotations(&self) -> Vec<Self> {
        let mut rotations = Vec::<Self>::new();
        let up = vec![Axis::X, Axis::Y, Axis::Z];
        let forward = vec![Axis::Z, Axis::X, Axis::Y];
        let mut scanner = self.clone();
        for (&up, forward) in up.iter().zip(forward) {
            for _ in 0..4 {
                for _ in 0..2 {
                    rotations.push(scanner.clone());
                    for _ in 0..2 {
                        scanner.rotate90(forward);
                    }
                }
                scanner.rotate90(up);
            }
            scanner.rotate90(forward);
        }
        rotations
    }

    fn rotate90(&mut self, axis: Axis) {
        for b in self.beacons.iter_mut() {
            *b = match axis {
                Axis::X => Vec3 {
                    x: b.x,
                    y: -b.z,
                    z: b.y,
                },
                Axis::Y => Vec3 {
                    x: b.z,
                    y: b.y,
                    z: -b.x,
                },
                Axis::Z => Vec3 {
                    x: -b.y,
                    y: b.x,
                    z: b.z,
                },
            };
        }
    }

    fn beacon_pairs(&self) -> impl Iterator<Item = (&Vec3, &Vec3)> {
        self.beacons
            .iter()
            .map(move |b1| {
                self.beacons
                    .iter()
                    .map(move |b2| (b1, b2))
                    .filter(|(b1, b2)| b1 != b2)
            })
            .flatten()
    }

    fn center_on(&mut self, other: &Self) {
        let common = self.find_common(&Common::default());
        let common = other.find_common(&common);
        self.center = common
            .values()
            .find_map(|(start, _)| {
                if start.len() > 1 {
                    Some(start[1] - start[0])
                } else {
                    None
                }
            })
            .unwrap();
        for b in self.beacons.iter_mut() {
            *b = *b + self.center;
        }
    }
}

type Common = HashMap<Vec3, (Vec<Vec3>, Vec<Vec3>)>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash, Eq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
    Z,
}
