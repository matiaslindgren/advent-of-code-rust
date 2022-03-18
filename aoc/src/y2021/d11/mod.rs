use std::str;

pub fn main(input: &str) -> String {
    let mut map = input.parse::<Map>().expect("failed parsing map");
    let mut a = 0;
    let mut b = 0;
    let mut num_flashed = 0;
    while num_flashed < map.map.len() {
        num_flashed = 0;
        map.step();
        while map.ready_to_flash().count() > 0 {
            map = map.flash_all();
        }
        for energy in map.map.iter_mut() {
            if *energy < 0 {
                *energy = 0;
                num_flashed += 1;
            }
        }
        a += num_flashed;
        b += 1;
    }
    format!("{} {}", a, b)
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Clone)]
struct Map {
    map:   Vec<i8>,
    width: i64,
}

impl str::FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = vec![];
        let mut width = 0;
        for line in s.lines() {
            width = line.len() as i64;
            for ch in line.chars() {
                let x = ch
                    .to_digit(10)
                    .ok_or(format!("failed parsing digit '{}'", ch))?;
                map.push(x as i8);
            }
        }
        Ok(Self { map, width })
    }
}

impl Map {
    fn step(&mut self) {
        for energy in self.map.iter_mut() {
            *energy += 1;
        }
    }

    fn ready_to_flash(&self) -> impl Iterator<Item = usize> + '_ {
        self.map.iter().enumerate().filter_map(|(i, &energy)| {
            if energy > 9 {
                Some(i)
            } else {
                None
            }
        })
    }

    fn adjacent(&self, i: usize) -> Vec<usize> {
        let w = self.width as i64;
        let h = (self.map.len() as i64) / w;
        let y = (i as i64) / w;
        let x = (i as i64) % w;
        [
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x - 1),
            (y, x + 1),
            (y + 1, x - 1),
            (y + 1, x),
            (y + 1, x + 1),
        ]
        .iter()
        .filter_map(|&(y, x)| {
            if 0 <= x && x < w && 0 <= y && y < h {
                Some((y * self.width + x) as usize)
            } else {
                None
            }
        })
        .collect()
    }

    fn flash_all(&self) -> Self {
        let mut next = self.clone();
        for i in self.ready_to_flash() {
            next.map[i] = -1;
            for j in self.adjacent(i).into_iter() {
                if next.map[j] != -1 {
                    next.map[j] += 1;
                }
            }
        }
        next
    }
}
