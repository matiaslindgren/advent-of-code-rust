use crate::debug;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    Add,
    Mul,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    Less,
    Equal,
    End,
}

impl FromStr for Op {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u8>().unwrap() {
            1 => Ok(Op::Add),
            2 => Ok(Op::Mul),
            3 => Ok(Op::Input),
            4 => Ok(Op::Output),
            5 => Ok(Op::JumpTrue),
            6 => Ok(Op::JumpFalse),
            7 => Ok(Op::Less),
            8 => Ok(Op::Equal),
            99 => Ok(Op::End),
            _ => panic!("unknown op code {}", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    inputs: VecDeque<i64>,
    output: Option<i64>,
    pub terminated: bool,
    memory: Vec<String>,
    i_ins: usize,
}

impl IntCode {
    pub fn new(program: &str) -> Self {
        Self {
            memory: program
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<String>>()
                .to_vec(),
            inputs: [].into(),
            output: None,
            terminated: false,
            i_ins: 0,
        }
    }

    pub fn load(&self, i: usize) -> i64 {
        let x_str = &self.memory[i];
        match x_str.parse::<i64>() {
            Ok(x) => x,
            Err(e) => panic!("load failed for {}: {}", x_str, e),
        }
    }

    pub fn load_next(&mut self, mode: u8) -> i64 {
        let x = self.load_with_mode(self.i_ins, mode);
        self.i_ins += 1;
        x
    }

    fn load_with_mode(&self, i: usize, mode: u8) -> i64 {
        match mode {
            0 => self.load(self.load(i) as usize),
            1 => self.load(i),
            _ => panic!("load failed, unknown mode {}", mode),
        }
    }

    pub fn store(&mut self, i: usize, x: i64) {
        self.memory[i] = format!("{}", x);
    }

    #[cfg(debug_assertions)]
    pub fn dump_memory(&self) -> String {
        self.memory
            .iter()
            .enumerate()
            .map(|(line, instr)| format!("{:>3}:{:>6}", line, instr))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn next_instruction(&mut self) -> (Op, Vec<u8>) {
        let instr = &self.memory[self.i_ins];
        self.i_ins += 1;
        let (modes_str, op_str) = instr.split_at(instr.len().max(2) - 2);
        let op = Op::from_str(op_str).unwrap();
        let mut modes: Vec<u8> = modes_str.chars().map(|x| (x as u8) - 48).rev().collect();
        modes.resize(4, 0);
        (op, modes)
    }

    pub fn push_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    fn next_input(&mut self) -> i64 {
        match self.inputs.pop_front() {
            Some(input) => input,
            None => panic!("there are no inputs"),
        }
    }

    pub fn take_output(&mut self) -> i64 {
        match self.output.take() {
            Some(o) => o,
            None => panic!("there is no output"),
        }
    }

    fn bin_op(&self, x: i64, op: Op, y: i64) -> i64 {
        match op {
            Op::Add => x + y,
            Op::Mul => x * y,
            Op::Less => (x < y) as i64,
            Op::Equal => (x == y) as i64,
            _ => panic!(
                "cannot compute unknown binary operation {} {:?} {}",
                x, op, y
            ),
        }
    }

    fn jump_op(&self, op: Op, x: i64) -> bool {
        match op {
            Op::JumpTrue => x != 0,
            Op::JumpFalse => x == 0,
            _ => panic!("cannot compute unknown jump operation {:?} {}", op, x),
        }
    }

    pub fn step(&mut self) {
        let (op, modes) = self.next_instruction();
        debug!("{:>3}: {:?}", self.i_ins, op);
        match op {
            Op::Add | Op::Mul | Op::Less | Op::Equal => {
                let x1 = self.load_next(modes[0]);
                let x2 = self.load_next(modes[1]);
                let pos = self.load_next(1);
                self.store(pos as usize, self.bin_op(x1, op, x2));
            }
            Op::Input => {
                let pos = self.load_next(1);
                let r = self.next_input();
                self.store(pos as usize, r);
            }
            Op::Output => {
                self.output = Some(self.load_next(modes[0]));
            }
            Op::JumpTrue | Op::JumpFalse => {
                let x = self.load_next(modes[0]);
                if self.jump_op(op, x) {
                    self.i_ins = self.load_next(modes[1]) as usize;
                } else {
                    self.i_ins += 1;
                }
            }
            Op::End => {
                self.terminated = true;
            }
        };
    }

    pub fn run(&mut self) -> i64 {
        while !self.terminated && self.output.is_none() {
            self.step();
        }
        self.output.take().unwrap_or(0)
    }
}

pub fn run(program: &str, inputs: &[i64]) -> i64 {
    let mut ic = IntCode::new(program);
    for &inp in inputs {
        ic.push_input(inp);
    }
    ic.run()
}
