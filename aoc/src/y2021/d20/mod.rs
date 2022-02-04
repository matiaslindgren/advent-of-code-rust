use std::collections::HashMap;
use std::str::FromStr;

pub fn main(input: &str) -> String {
    let image = input.parse::<Image>().unwrap();
    let a = image.enhance(2).count_light();
    let b = image.enhance(50).count_light();
    format!("{} {}", a, b)
}

type Grid = HashMap<Vec2, u8>;
type Vec2 = (i64, i64);

#[derive(Debug, Clone)]
struct Image {
    grid:      Grid,
    algorithm: Vec<u8>,
}

impl FromStr for Image {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (algo_str, img_str) = input.split_once("\n\n").unwrap();
        let parse_pixel = |ch: char| (ch == '#') as u8;
        let algorithm: Vec<u8> = algo_str.chars().map(parse_pixel).collect();
        let mut grid = Grid::new();
        for (y, line) in img_str.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid.insert((x as i64, y as i64), parse_pixel(ch));
            }
        }
        Ok(Self { grid, algorithm })
    }
}

impl Image {
    fn enhance(&self, n: usize) -> Self {
        let mut result = self.clone();
        for step in 0..n {
            let default = if step % 2 == 0 { 0 } else { self.algorithm[0] };
            result = result.enhance_step(default);
        }
        result
    }

    fn enhance_step(&self, default: u8) -> Self {
        let ((min_x, min_y), (max_x, max_y)) = self.corners();
        let mut result = self.clone();
        for y in (min_y - 1)..=(max_y + 1) {
            for x in (min_x - 1)..=(max_x + 1) {
                let idx = self.extract_index(x, y, default);
                result.grid.insert((x, y), self.algorithm[idx]);
            }
        }
        result
    }

    fn corners(&self) -> (Vec2, Vec2) {
        let max = i64::max_value();
        let min = i64::min_value();
        self.grid.keys().cloned().fold(
            ((max, max), (min, min)),
            |((min_x, min_y), (max_x, max_y)), (x, y)| {
                ((min_x.min(x), min_y.min(y)), (max_x.max(x), max_y.max(y)))
            },
        )
    }

    fn extract_index(&self, x_mid: i64, y_mid: i64, default: u8) -> usize {
        let y_range = (y_mid - 1)..=(y_mid + 1);
        let x_range = (x_mid - 1)..=(x_mid + 1);
        let mut idx = 0;
        let mut i = 0;
        for y in y_range.rev() {
            for x in x_range.clone().rev() {
                let px = self.grid.get(&(x, y)).unwrap_or(&default);
                idx += (*px as usize) << i;
                i += 1;
            }
        }
        idx
    }

    fn count_light(&self) -> usize {
        self.grid.values().map(|&px| px as usize).sum()
    }
}
