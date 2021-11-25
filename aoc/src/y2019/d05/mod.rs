use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let prog = intcode::parse_program(input);
    let a = intcode::run(&mut prog.clone(), &vec![1]);
    let b = 0;
    format!("{} {}", a, b)
}
