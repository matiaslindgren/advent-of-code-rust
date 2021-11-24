pub fn run(mem: &mut [usize], noun: usize, verb: usize) -> usize {
    mem[1] = noun;
    mem[2] = verb;
    let mut i = 0;
    loop {
        let op = mem[i];
        match op {
            1 | 2 => {
                let a = mem[mem[i + 1]];
                let b = mem[mem[i + 2]];
                mem[mem[i + 3]] = match op {
                    1 => a + b,
                    2 => a * b,
                    _ => 0,
                };
            }
            99 => return mem[0],
            _ => panic!("unknown op code {}", op),
        };
        i += 4;
    }
}
