pub fn parse_opcode(input: &str) -> (u8, Vec<u8>) {
    let (modes_str, op_str) = input.split_at(input.len().max(2) - 2);
    let op = op_str.parse::<u8>().expect("failed parsing opcode");
    let mut modes: Vec<u8> = modes_str.chars().map(|x| (x as u8) - 48).collect();
    modes.resize(4, 0);
    (op, modes)
}

fn load(mem: &[String], i: usize, mode: u8) -> i64 {
    println!("load({}) mem[{}] = {}", mode, i, mem[i]);
    let mut x = mem[i].parse::<i64>().expect("failed parsing element");
    if mode == 0 {
        x = load(mem, x as usize, 1);
    }
    x
}

pub fn parse_program(s: &str) -> Vec<String> {
    s.split(',').map(str::to_owned).collect()
}

pub fn run(mem: &mut [String], inputs: &[i64]) -> String {
    let mut outputs = Vec::<i64>::new();
    let mut i_ins = 0;
    let mut i_inp = 0;
    loop {
        let (op, modes) = parse_opcode(&mem[i_ins]);
        println!("op {} modes {:?}", op, modes);
        i_ins += 1;
        match op {
            1 | 2 => {
                let x1 = load(mem, i_ins, modes[0]);
                i_ins += 1;
                let x2 = load(mem, i_ins, modes[1]);
                i_ins += 1;
                let r = match op {
                    1 => x1 + x2,
                    2 => x1 * x2,
                    _ => 0,
                };
                let pos = load(mem, i_ins, 1);
                i_ins += 1;
                mem[pos as usize] = format!("{}", r);
            }
            3 => {
                let pos = load(mem, i_ins, 1);
                i_ins += 1;
                let inp = inputs[i_inp];
                i_inp += 1;
                mem[pos as usize] = format!("{}", inp);
            }
            4 => {
                let x = load(mem, i_ins, 1);
                i_ins += 1;
                outputs.push(x);
            }
            99 => break,
            _ => panic!("unknown op code {}", op),
        };
    }
    mem[0].to_owned()
}
