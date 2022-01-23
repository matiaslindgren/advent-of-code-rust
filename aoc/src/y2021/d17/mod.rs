pub fn main(input: &str) -> String {
    let (target_x, target_y) = parse_target_area(input);
    let a = find_a(target_y);
    let b = find_b((target_x, target_y));
    format!("{} {}", a, b)
}

type Vec2 = (i32, i32);

fn parse_target_area(s: &str) -> (Vec2, Vec2) {
    let (_, s) = s.split_once("target area: ").unwrap();
    let (x, y) = s.split_once(", ").unwrap();
    (parse_range(x), parse_range(y))
}

fn parse_range(s: &str) -> Vec2 {
    let (_, s) = s.split_once('=').unwrap();
    let (a, b) = s.split_once("..").unwrap();
    (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
}

fn find_a(target_y: Vec2) -> i32 {
    (target_y.0..target_y.0.abs())
        .filter_map(|v_y| find_max_y_pos(target_y, v_y))
        .max()
        .unwrap()
}

fn find_b(target: (Vec2, Vec2)) -> usize {
    let (target_x, target_y) = target;
    let mut n = 0;
    for v_x in 0..=target_x.1 {
        for v_y in target_y.0..target_y.0.abs() {
            n += reaches_target(target, v_x, v_y) as usize;
        }
    }
    n
}

fn find_max_y_pos(target: Vec2, v_0: i32) -> Option<i32> {
    let mut y = 0;
    let mut y_max = 0;
    let mut v_y = v_0;
    loop {
        if y < target.0 {
            break None;
        }
        if inside_target(target, y) {
            break Some(y_max);
        }
        y += v_y;
        y_max = y_max.max(y);
        v_y -= 1;
    }
}

fn inside_target(target: Vec2, pos: i32) -> bool {
    let (begin, end) = target;
    begin <= pos && pos <= end
}

fn reaches_target(target: (Vec2, Vec2), v0_x: i32, v0_y: i32) -> bool {
    let mut x = 0;
    let mut y = 0;
    let mut v_x = v0_x;
    let mut v_y = v0_y;
    let (target_x, target_y) = target;
    loop {
        if x > target_x.1 || y < target_y.0 {
            break false;
        }
        if inside_target(target_x, x) && inside_target(target_y, y) {
            break true;
        }
        x += v_x;
        y += v_y;
        v_x -= v_x.signum();
        v_y -= 1;
    }
}
