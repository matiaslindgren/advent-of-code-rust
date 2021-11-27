use crate::y2019::intcode;

pub fn main(input: &str) -> String {
    let a = find_a(input, "12", "2");
    let b = find_b(input);
    format!("{} {}", a, b)
}

fn find_a(input: &str, noun: &str, verb: &str) -> String {
    let mut input: Vec<&str> = input.splitn(4, ',').collect();
    input[1] = noun;
    input[2] = verb;
    let (mem, _) = intcode::run(&input.join(","), &[]);
    mem
}

fn find_b(v: &str) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let noun_str = format!("{}", noun);
            let verb_str = format!("{}", verb);
            if find_a(v, &noun_str, &verb_str) == "19690720" {
                return 100 * noun + verb;
            }
        }
    }
    0
}
