use crate::common;
use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let v = common::numbers::<usize>(input, ",");
    let a = intcode::run(&mut v.clone(), 12, 2);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_b(v: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let res = intcode::run(&mut v.to_vec(), noun, verb);
            if res == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
