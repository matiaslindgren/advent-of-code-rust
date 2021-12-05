use crate::common::Vec3;
use crate::math::lcm;
use std::collections::HashSet;

pub fn main(input: &str) -> String {
    let system = System {
        moons: parse_moons(input),
    };
    let a = find_a(&system);
    let b = find_b(&system);
    format!("{} {}", a, b)
}

#[derive(Clone, Copy)]
struct Moon {
    p: Vec3,
    v: Vec3,
}

#[derive(Clone)]
struct System {
    pub moons: Vec<Moon>,
}

impl System {
    fn step(&mut self) {
        let n = self.moons.len();
        for i in 0..n {
            for j in (i + 1)..n {
                let m1 = &self.moons[i];
                let m2 = &self.moons[j];
                let gravity = (m1.p - m2.p).signum();
                let mut m1 = &mut self.moons[i];
                m1.v = m1.v - gravity;
                let mut m2 = &mut self.moons[j];
                m2.v = m2.v + gravity;
            }
        }
        for m in self.moons.iter_mut() {
            m.p = m.p + m.v;
        }
    }

    fn energy(&self) -> i64 {
        self.moons.iter().map(|m| m.p.l1() * m.v.l1()).sum()
    }
}

fn parse_moons(input: &str) -> Vec<Moon> {
    let drop_chars = &['<', 'x', 'y', 'z', ' ', '=', '>'][..];
    let parse_moon = |line: &str| {
        let mut xyz = line
            .split(',')
            .map(|s| s.replace(drop_chars, "").parse::<i64>().unwrap());
        Moon {
            p: Vec3 {
                x: xyz.next().unwrap(),
                y: xyz.next().unwrap(),
                z: xyz.next().unwrap(),
            },
            v: Vec3::default(),
        }
    };
    input.lines().map(parse_moon).collect()
}

fn find_a(system: &System) -> i64 {
    let mut system = system.clone();
    for _ in 0..1000 {
        system.step();
    }
    system.energy()
}

#[derive(Default, Clone)]
struct Trajectories {
    x:    HashSet<String>,
    y:    HashSet<String>,
    z:    HashSet<String>,
    xlen: Option<usize>,
    ylen: Option<usize>,
    zlen: Option<usize>,
}

impl Trajectories {
    fn push(&mut self, moons: &[Moon], step: usize) {
        let (x, (y, z)): (String, (String, String)) = moons
            .iter()
            .map(|m| {
                (
                    format!("{} {} ", m.p.x, m.v.x),
                    (
                        format!("{} {} ", m.p.y, m.v.y),
                        format!("{} {} ", m.p.z, m.v.z),
                    ),
                )
            })
            .unzip();
        if self.xlen.is_none() && self.x.contains(&x) {
            self.xlen = Some(step);
        }
        if self.ylen.is_none() && self.y.contains(&y) {
            self.ylen = Some(step);
        }
        if self.zlen.is_none() && self.z.contains(&z) {
            self.zlen = Some(step);
        }
        self.x.insert(x);
        self.y.insert(y);
        self.z.insert(z);
    }

    fn incomplete(&self) -> bool {
        self.xlen.is_none() || self.ylen.is_none() || self.zlen.is_none()
    }

    fn cycle_lengths(&self) -> Vec<i64> {
        [self.xlen, self.ylen, self.zlen]
            .iter()
            .map(|&c| c.unwrap() as i64)
            .collect()
    }
}

fn find_b(system: &System) -> i64 {
    let mut system = system.clone();
    let mut trajectories = Trajectories::default();
    let mut step = 0;
    while trajectories.incomplete() {
        trajectories.push(&system.moons, step);
        system.step();
        step += 1;
    }
    trajectories
        .cycle_lengths()
        .iter()
        .fold(1, |n, &cycle_len| lcm(n, cycle_len))
}
