use crate::debug;

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
    Break,
}

fn parse_opcode(input: &str) -> (Op, Vec<u8>) {
    let (modes_str, op_str) = input.split_at(input.len().max(2) - 2);
    let op = match op_str.parse::<u8>().expect("failed parsing opcode") {
        1 => Op::Add,
        2 => Op::Mul,
        3 => Op::Input,
        4 => Op::Output,
        5 => Op::JumpTrue,
        6 => Op::JumpFalse,
        7 => Op::Less,
        8 => Op::Equal,
        99 => Op::Break,
        _ => panic!("unknown op code {}", op_str),
    };
    let mut modes: Vec<u8> = modes_str.chars().map(|x| (x as u8) - 48).rev().collect();
    modes.resize(4, 0);
    (op, modes)
}

fn load(mem: &[String], i: usize, mode: u8) -> i64 {
    debug!("load mem[{}]({}) m{}", i, mem[i], mode);
    let mut x = mem[i].parse::<i64>().expect("failed parsing element");
    if mode == 0 {
        assert!(x >= 0, "{}", x);
        x = load(mem, x as usize, 1);
    }
    x
}

fn store(mem: &mut [String], i: i64, value: i64) {
    debug!("store mem[{}]({}) := {}", i, mem[i as usize], value);
    mem[i as usize] = format!("{}", value);
}

#[cfg(debug_assertions)]
fn dump_memory(mem: &[String]) -> String {
    mem.iter()
        .enumerate()
        .map(|(line, instr)| format!("{:>3}:{:>6}", line, instr))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn run(program: &str, inputs: &[i64]) -> (String, i64) {
    let mut mem = program
        .split(',')
        .map(str::to_owned)
        .collect::<Vec<String>>()
        .to_vec();
    debug!("{}", dump_memory(&mem));
    let mut output = 0;
    let mut i_ins = 0;
    let mut i_inp = 0;
    loop {
        let (op, modes) = parse_opcode(&mem[i_ins]);
        debug!("{}: op {:?} modes {:?}", i_ins, op, modes);
        i_ins += 1;
        match op {
            Op::Add | Op::Mul => {
                let x1 = load(&mem, i_ins, modes[0]);
                i_ins += 1;
                let x2 = load(&mem, i_ins, modes[1]);
                i_ins += 1;
                let r = match op {
                    Op::Add => x1 + x2,
                    Op::Mul => x1 * x2,
                    _ => 0,
                };
                let pos = load(&mem, i_ins, 1);
                i_ins += 1;
                store(&mut mem, pos, r);
            }
            Op::Input => {
                let pos = load(&mem, i_ins, 1);
                i_ins += 1;
                let inp = inputs[i_inp];
                i_inp += 1;
                store(&mut mem, pos, inp);
            }
            Op::Output => {
                output = load(&mem, i_ins, modes[0]);
                i_ins += 1;
                debug!("output {:?}", output);
            }
            Op::JumpTrue | Op::JumpFalse => {
                let x1 = load(&mem, i_ins, modes[0]);
                i_ins += 1;
                let jump = (op == Op::JumpTrue && x1 != 0) || (op == Op::JumpFalse && x1 == 0);
                if jump {
                    let x2 = load(&mem, i_ins, modes[1]);
                    i_ins = x2 as usize;
                } else {
                    i_ins += 1;
                }
            }
            Op::Less | Op::Equal => {
                let x1 = load(&mem, i_ins, modes[0]);
                i_ins += 1;
                let x2 = load(&mem, i_ins, modes[1]);
                i_ins += 1;
                let pos = load(&mem, i_ins, 1);
                i_ins += 1;
                let r = (op == Op::Less && x1 < x2) || (op == Op::Equal && x1 == x2);
                store(&mut mem, pos, r as i64);
            }
            Op::Break => break,
        };
    }
    (mem[0].to_owned(), output)
}

#[test]
fn test_parse_opcode_only() {
    let op_str_list = vec!["1", "01", "2", "02", "99"];
    let expected_ops = vec![Op::Add, Op::Add, Op::Mul, Op::Mul, Op::Break];
    for (op_str, expect_op) in op_str_list.iter().zip(expected_ops) {
        let (op, modes) = parse_opcode(op_str);
        assert_eq!(op, expect_op);
        assert_eq!(modes.len(), 4);
        for m in modes {
            assert_eq!(m, 0);
        }
    }
}

#[test]
fn test_parse_opcode_modes1() {
    let (op, modes) = parse_opcode("11199");
    assert_eq!(op, Op::Break);
    assert_eq!(modes.len(), 4);
    assert_eq!(modes[0], 1);
    assert_eq!(modes[1], 1);
    assert_eq!(modes[2], 1);
    assert_eq!(modes[3], 0);
}

#[test]
fn test_parse_opcode_modes2() {
    let (op, modes) = parse_opcode("1001");
    assert_eq!(op, Op::Add);
    assert_eq!(modes.len(), 4);
    assert_eq!(modes[0], 0);
    assert_eq!(modes[1], 1);
    assert_eq!(modes[2], 0);
    assert_eq!(modes[3], 0);
}
