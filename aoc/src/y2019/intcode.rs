fn parse_opcode(input: &str) -> (u8, Vec<u8>) {
    let (modes_str, op_str) = input.split_at(input.len().max(2) - 2);
    let op = op_str.parse::<u8>().expect("failed parsing opcode");
    let mut modes: Vec<u8> = modes_str.chars().map(|x| (x as u8) - 48).collect();
    modes.resize(4, 0);
    (op, modes)
}

fn parse(mem: &[String], i: usize, mode: u8) -> usize {
    let mut x = mem[i].parse::<usize>().expect("failed parsing element");
    if mode == 0 {
        x = parse(mem, x, 1);
    }
    x
}

pub fn run(mem: &mut [String]) -> String {
    let mut i = 0;
    loop {
        let (op, modes) = parse_opcode(&mem[i]);
        match op {
            1 | 2 => {
                let a = parse(mem, i + 1, modes[0]);
                let b = parse(mem, i + 2, modes[1]);
                let r = match op {
                    1 => a + b,
                    2 => a * b,
                    _ => 0,
                };
                let pos = parse(mem, i + 3, 1);
                mem[pos] = format!("{}", r);
                i += 3;
            }
            99 => break,
            _ => panic!("unknown op code {}", op),
        };
        i += 1;
    }
    mem[0].to_owned()
}
