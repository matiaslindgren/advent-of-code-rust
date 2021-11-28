use crate::y2019::intcode::IntCode;

pub fn main(input: &str) -> String {
    let a = find_a(input, 12, 2);
    let b = find_b(input);
    format!("{} {}", a, b)
}

fn find_a(input: &str, noun: i64, verb: i64) -> i64 {
    let mut prog = IntCode::new(input);
    prog.store(1, noun);
    prog.store(2, verb);
    while !prog.terminated {
        prog.step();
    }
    prog.load(0)
}

fn find_b(input: &str) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if find_a(input, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
