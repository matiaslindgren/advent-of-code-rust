use crate::debug;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
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
    UpdateRelBase,
    End,
}

impl FromStr for Op {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match str::parse::<u8>(s)? {
            1 => Op::Add,
            2 => Op::Mul,
            3 => Op::Input,
            4 => Op::Output,
            5 => Op::JumpTrue,
            6 => Op::JumpFalse,
            7 => Op::Less,
            8 => Op::Equal,
            9 => Op::UpdateRelBase,
            99 => Op::End,
            _ => panic!("unknown op code {}", s),
        };
        Ok(op)
    }
}

#[derive(Debug, Clone)]
pub struct IntCode {
    inputs:   VecDeque<i64>,
    output:   Option<i64>,
    pub done: bool,
    memory:   HashMap<usize, String>,
    i_ins:    usize,
    rel_base: i64,
}

impl IntCode {
    pub fn new(program: &str) -> Self {
        Self {
            memory:   program
                .split(',')
                .map(str::to_owned)
                .enumerate()
                .collect(),
            inputs:   [].into(),
            output:   None,
            done:     false,
            i_ins:    0,
            rel_base: 0,
        }
    }

    pub fn load<T>(&self, i: usize) -> T
    where
        T: Default + FromStr,
    {
        match self.memory.get(&i) {
            Some(x_str) => str::parse::<T>(x_str)
                .unwrap_or_else(|_| panic!("failed loading {}", x_str)),
            None => T::default(),
        }
    }

    fn load_m(&self, i: i64, mode: u8) -> i64 {
        debug!("    load[{}] m{}", i, mode);
        match mode {
            0 => self.load::<i64>(i as usize),
            1 => i,
            2 => self.load::<i64>((self.rel_base + i) as usize),
            _ => panic!("unknown load mode {}", mode),
        }
    }

    fn store_m(&mut self, i: i64, x: i64, mode: u8) {
        let pos = match mode {
            0 => i,
            2 => self.rel_base + i as i64,
            _ => panic!("unknown store mode {}", mode),
        };
        self.memory.insert(pos as usize, format!("{}", x));
    }

    pub fn store(&mut self, i: i64, x: i64) {
        self.store_m(i, x, 0);
    }

    fn next_param(&mut self) -> i64 {
        let i = self.i_ins;
        self.i_ins += 1;
        self.load::<i64>(i)
    }

    fn next_instruction(&mut self) -> (Op, Vec<u8>) {
        let instr = self.load::<String>(self.i_ins);
        self.i_ins += 1;
        let (modes_str, op_str) = instr.split_at(instr.len().max(2) - 2);
        let op = Op::from_str(op_str).unwrap();
        let mut modes: Vec<u8> =
            modes_str.chars().map(|x| (x as u8) - 48).rev().collect();
        modes.resize(3, 0);
        (op, modes)
    }

    fn bin_op(&self, x: i64, op: &Op, y: i64) -> i64 {
        match op {
            Op::Add => x + y,
            Op::Mul => x * y,
            Op::Less => (x < y) as i64,
            Op::Equal => (x == y) as i64,
            _ => panic!("undefined binary op {:?}", op),
        }
    }

    fn jump_op(&self, op: &Op, flag: i64) -> bool {
        match op {
            Op::JumpTrue => flag != 0,
            Op::JumpFalse => flag == 0,
            _ => panic!("undefined jump op {:?}", op),
        }
    }

    pub fn push_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    fn next_input(&mut self) -> i64 {
        self.inputs.pop_front().expect("there are no inputs")
    }

    pub fn take_output(&mut self) -> i64 {
        self.output.take().expect("there is no output")
    }

    pub fn step(&mut self) {
        let (op, modes) = self.next_instruction();
        debug!("{:>3}: {:?} {:?}", self.i_ins, op, modes);
        match op {
            Op::Add | Op::Mul | Op::Less | Op::Equal => {
                let (p1, p2) = (self.next_param(), self.next_param());
                let x1 = self.load_m(p1, modes[0]);
                let x2 = self.load_m(p2, modes[1]);
                let res = self.bin_op(x1, &op, x2);
                let pos = self.next_param();
                self.store_m(pos, res, modes[2]);
                debug!(
                    "    m[{}] := {:?}({}, {}) (= {})",
                    pos, op, x1, x2, res
                );
            }
            Op::Input => {
                let pos = self.next_param();
                let input = self.next_input();
                self.store_m(pos, input, modes[0]);
                debug!("    m[{}] := {}", pos, input);
            }
            Op::Output => {
                let p = self.next_param();
                let x = self.load_m(p, modes[0]);
                self.output = Some(x);
                debug!("    {:?}", self.output);
            }
            Op::JumpTrue | Op::JumpFalse => {
                let p1 = self.next_param();
                let flag = self.load_m(p1, modes[0]);
                if self.jump_op(&op, flag) {
                    let p2 = self.next_param();
                    let pos = self.load_m(p2, modes[1]);
                    self.i_ins = pos as usize;
                    debug!("    {:?} -> {}", op, self.i_ins);
                } else {
                    self.i_ins += 1;
                }
            }
            Op::UpdateRelBase => {
                let p = self.next_param();
                self.rel_base += self.load_m(p, modes[0]);
                debug!("    {}", self.rel_base);
            }
            Op::End => {
                self.done = true;
            }
        };
    }

    pub fn run(&mut self) -> Option<i64> {
        while !self.done && self.output.is_none() {
            self.step();
        }
        self.output.take()
    }

    pub fn dump_memory(&self) -> String {
        let mut keys: Vec<&usize> = self.memory.keys().collect();
        keys.sort_unstable();
        let lines: Vec<String> = keys
            .iter()
            .map(|&k| format!("{:>4}: {}", k, self.memory.get(k).unwrap()))
            .collect();
        lines.join("\n")
    }
}

pub fn run(program: &str, inputs: &[i64]) -> i64 {
    let mut ic = IntCode::new(program);
    for &inp in inputs {
        ic.push_input(inp);
    }
    ic.run().unwrap_or(0)
}
