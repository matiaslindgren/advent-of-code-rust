use std::str::FromStr;

pub fn main(input: &str) -> String {
    let steps = parse_input(input);
    let a = reboot(steps.iter().filter(|s| s.cuboid.is_init()));
    let b = reboot(steps.iter());
    format!("{} {}", a, b)
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(str::parse::<Step>)
        .map(Result::unwrap)
        .collect()
}

/// Inclusion-exclusion principle where set cardinalities are cuboid volumes.
/// If cuboid step is off, exclude cardinality from sum.
fn reboot<'a>(steps: impl Iterator<Item = &'a Step>) -> i64 {
    let mut intersections = Intersections::new();
    for step in steps {
        let cuboid1 = &step.cuboid;
        let mut new_intersections = vec![(step.on as i64, cuboid1.clone())];
        for (sign, cuboid2) in intersections.iter() {
            if let Some(intersection) = cuboid1.intersection(cuboid2) {
                new_intersections.push((-1 * sign, intersection));
            }
        }
        intersections.extend_from_slice(&new_intersections);
    }
    intersections
        .into_iter()
        .map(|(sign, cuboid)| sign * cuboid.volume())
        .sum()
}

type Intersection = (i64, Cuboid);
type Intersections = Vec<Intersection>;
type Range = (i64, i64);

#[derive(Debug, Clone)]
struct Step {
    on:     bool,
    cuboid: Cuboid,
}

#[derive(Debug, Clone)]
struct Cuboid {
    x: Range,
    y: Range,
    z: Range,
}

impl Cuboid {
    fn is_init(&self) -> bool {
        [self.x.0, self.x.1, self.y.0, self.y.1, self.z.0, self.z.1]
            .iter()
            .all(|x| x.abs() <= 50)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        let x = (self.x.0.max(other.x.0), self.x.1.min(other.x.1));
        let y = (self.y.0.max(other.y.0), self.y.1.min(other.y.1));
        let z = (self.z.0.max(other.z.0), self.z.1.min(other.z.1));
        if x.0 <= x.1 && y.0 <= y.1 && z.0 <= z.1 {
            Some(Self { x, y, z })
        } else {
            None
        }
    }

    fn volume(&self) -> i64 {
        let x = (self.x.0 - self.x.1).abs() + 1;
        let y = (self.y.0 - self.y.1).abs() + 1;
        let z = (self.z.0 - self.z.1).abs() + 1;
        x * y * z
    }
}

impl FromStr for Step {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (on, xyz) = s.split_once(' ').unwrap();
        let xyz: Vec<Range> = xyz
            .split(',')
            .map(|s| {
                let (_, range) = s.split_once('=').unwrap();
                let ab: Vec<i64> = range
                    .split("..")
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect();
                (ab[0], ab[1])
            })
            .collect();
        Ok(Self {
            on:     on == "on",
            cuboid: Cuboid {
                x: xyz[0],
                y: xyz[1],
                z: xyz[2],
            },
        })
    }
}
