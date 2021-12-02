use crate::grid::Grid;

pub fn main(input: &str) -> String {
    let v: Vec<char> = input.chars().collect();
    let size = (6, 25);
    let a = find_a(&v, size);
    let b = find_b(&v, size);
    format!("{}\n{}", a, b)
}

fn find_layers(v: &[char], h: usize, w: usize) -> Vec<Vec<char>> {
    v.chunks_exact(h * w)
        .map(|layer_data| layer_data.to_vec())
        .collect()
}

fn count_layer(layer: &[char], ch: char) -> usize {
    layer.iter().filter(|&&x| x == ch).count()
}

fn find_a(v: &[char], size: (usize, usize)) -> usize {
    let (h, w) = size;
    let layers = find_layers(v, h, w);
    let layer = layers
        .iter()
        .min_by_key(|&layer| count_layer(layer, '0'))
        .unwrap();
    count_layer(layer, '1') * count_layer(layer, '2')
}

fn find_b(v: &[char], size: (usize, usize)) -> String {
    let (h, w) = size;
    let layers = find_layers(v, h, w);
    let mut grid = Grid::<char>::new(h, w);
    for y in 0..h {
        for x in 0..w {
            let px = layers
                .iter()
                .map(|layer| layer[y * w + x])
                .find(|&ch| ch != '2')
                .unwrap_or('2');
            let ch = if px == '0' { ' ' } else { px };
            grid.set((y as i64, x as i64), ch);
        }
    }
    grid.to_string()
}
