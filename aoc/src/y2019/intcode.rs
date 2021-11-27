use std::num::ParseIntError;
use std::str::FromStr;

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

pub struct IntCode {
    inputs: Vec<i64>,
    output: Option<i64>,
    pub terminated: bool,
    memory: Vec<String>,
    i_ins: usize,
    i_inp: usize,
}

impl IntCode {
    pub fn new(program: &str, inputs: &[i64]) -> Self {
        Self {
            memory: program
                .split(',')
                .map(str::to_owned)
                .collect::<Vec<String>>()
                .to_vec(),
            inputs: inputs.to_vec(),
            output: None,
            terminated: false,
            i_ins: 0,
            i_inp: 0,
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

    fn next_instruction(&self, instr: &str) -> (Op, Vec<u8>) {
        let (modes_str, op_str) = instr.split_at(instr.len().max(2) - 2);
        let op = Op::from_str(op_str).unwrap();
        let mut modes: Vec<u8> = modes_str.chars().map(|x| (x as u8) - 48).rev().collect();
        modes.resize(4, 0);
        (op, modes)
    }

    pub fn read_output(&self) -> i64 {
        match self.output {
            Some(o) => o,
            None => panic!("there is no output"),
        }
    }

    pub fn run_until_end(program: &str, inputs: &[i64]) -> i64 {
        let mut code = Self::new(program, inputs);
        while !code.terminated {
            code.step();
        }
        code.read_output()
    }

    pub fn step(&mut self) {
        let (op, modes) = self.next_instruction(&self.memory[self.i_ins]);
        self.i_ins += 1;
        match op {
            Op::Add | Op::Mul => {
                let x1 = self.load_next(modes[0]);
                let x2 = self.load_next(modes[1]);
                let r = match op {
                    Op::Add => x1 + x2,
                    Op::Mul => x1 * x2,
                    _ => 0,
                };
                let pos = self.load_next(1);
                self.store(pos as usize, r);
            }
            Op::Input => {
                let pos = self.load_next(1);
                let r = self.inputs[self.i_inp];
                self.i_inp += 1;
                self.store(pos as usize, r);
            }
            Op::Output => {
                self.output = Some(self.load_next(modes[0]));
            }
            Op::JumpTrue | Op::JumpFalse => {
                let flag = self.load_next(modes[0]) != 0;
                if (op == Op::JumpTrue && flag) || (op == Op::JumpFalse && !flag) {
                    self.i_ins = self.load_next(modes[1]) as usize;
                } else {
                    self.i_ins += 1;
                }
            }
            Op::Less | Op::Equal => {
                let x1 = self.load_next(modes[0]);
                let x2 = self.load_next(modes[1]);
                let pos = self.load_next(1);
                let r = (op == Op::Less && x1 < x2) || (op == Op::Equal && x1 == x2);
                self.store(pos as usize, r as i64);
            }
            Op::End => {
                self.terminated = true;
            }
        };
    }
}
