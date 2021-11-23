use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<usize>(input, ",");
    format!("{} {}", find_a(&mut v.clone(), 12, 2), find_b(&v))
}

fn find_a(v: &mut [usize], noun: usize, verb: usize) -> usize {
    v[1] = noun;
    v[2] = verb;
    let mut i = 0;
    loop {
        let op = v[i];
        match op {
            1 | 2 => {
                let a = v[v[i + 1]];
                let b = v[v[i + 2]];
                v[v[i + 3]] = match op {
                    1 => a + b,
                    2 => a * b,
                    _ => 0,
                };
            }
            99 => return v[0],
            _ => panic!("unknown op code {}", op),
        };
        i += 4;
    }
}

fn find_b(v: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let res = find_a(&mut v.to_vec(), noun, verb);
            if res == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
