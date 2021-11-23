use crate::common;
use std::collections::HashMap;

type WirePaths = HashMap<(i32, i32), (char, u32)>;

pub fn main(input: &str) -> String {
    let (wire1, wire2) = input.split_once('\n').unwrap();
    let wire1 = common::directions::<char, i32>(wire1, ",", 1);
    let wire2 = common::directions::<char, i32>(wire2, ",", 1);
    let mut wire_paths = WirePaths::new();
    find_paths(&wire1, '1', &mut wire_paths);
    find_paths(&wire2, '2', &mut wire_paths);
    let mut a = u32::MAX;
    let mut b = u32::MAX;
    for ((y, x), (m, steps)) in wire_paths {
        if m == 'X' {
            a = a.min((y.abs() + x.abs()) as u32);
            b = b.min(steps);
        }
    }
    format!("{} {}", a, b)
}

fn find_paths(wire: &[(char, i32)], mark: char, paths: &mut WirePaths) {
    let mut y = 0;
    let mut x = 0;
    let mut total_steps = 0;
    for &(direction, steps) in wire {
        for _ in 0..steps {
            match direction {
                'U' => y += 1,
                'R' => x += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                _ => panic!("unknown direction {:?}", direction),
            };
            total_steps += 1;
            match paths.get(&(y, x)) {
                Some(&(mark_prev, total_steps_prev)) => {
                    if mark_prev != 'X' && mark_prev != mark {
                        paths.insert((y, x), ('X', total_steps + total_steps_prev))
                    } else {
                        None
                    }
                }
                None => paths.insert((y, x), (mark, total_steps)),
            };
        }
    }
}
