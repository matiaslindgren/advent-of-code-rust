use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let prog = intcode::parse_program(input);
    let (_, outputs) = intcode::run(&prog, &[1]);
    let b = 0;
    format!("{} {}", outputs[outputs.len() - 1], b)
}
