use crate::y2019::intcode::IntCode;

pub fn main(input: &str) -> String {
    let a = run(input, 1);
    let b = run(input, 2);
    format!("{} {}", a, b)
}

fn run(prog: &str, input: i64) -> i64 {
    let mut ic = IntCode::new(prog);
    ic.push_input(input);
    let mut output = 0;
    while !ic.terminated {
        if let Some(out) = ic.run() {
            output = out;
        }
    }
    output
}
