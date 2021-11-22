use crate::common;

pub fn main(input: &str) -> String {
    let v = common::numbers::<u32>(input, ",");
    format!("{} {}", find_a(&mut v.clone(), 12, 2), find_b(&v))
}

fn find_a(v: &mut [u32], noun: u32, verb: u32) -> u32 {
    v[1] = noun;
    v[2] = verb;
    let mut i = 0;
    loop {
        let op = v[i];
        match op {
            1 | 2 => {
                let a = v[v[i + 1] as usize];
                let b = v[v[i + 2] as usize];
                let r = &mut v[v[i + 3] as usize];
                match op {
                    1 => *r = a + b,
                    2 => *r = a * b,
                    _ => (),
                };
            }
            99 => return v[0],
            _ => panic!("unknown op code {}", op),
        };
        i += 4;
    }
}

fn find_b(v: &[u32]) -> u32 {
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
