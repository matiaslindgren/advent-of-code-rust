use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let v: Vec<String> = input.split(',').map(str::to_owned).collect();
    let a = find_a(&v, 12, 2);
    let b = find_b(&v);
    format!("{} {}", a, b)
}

fn find_a(v: &[String], noun: usize, verb: usize) -> String {
    let mut mem = v.to_vec();
    mem[1] = format!("{}", noun);
    mem[2] = format!("{}", verb);
    intcode::run(&mut mem)
}

fn find_b(v: &[String]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if find_a(v, noun, verb) == "19690720" {
                return 100 * noun + verb;
            }
        }
    }
    0
}
