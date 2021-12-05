use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let a = find_a(input);
    let b = intcode::run(input, &[5]);
    format!("{} {}", a, b)
}

pub fn find_a(input: &str) -> i64 {
    let mut ic = intcode::IntCode::new(input);
    ic.push_input(1);
    while !ic.done {
        ic.step();
    }
    ic.take_output()
}
