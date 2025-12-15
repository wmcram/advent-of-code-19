use std::collections::VecDeque;

use itertools::Itertools;

enum Step {
    AwaitingInput,
    Output(i64),
    Halt,
}

struct IntcodeMachine {
    memory: Vec<i64>,
    input: VecDeque<i64>,
    ip: usize,
    halted: bool,
}

enum Operation {
    Halt,
    Add(i64, i64, usize),
    Mul(i64, i64, usize),
    In(usize),
    Out(i64),
    JmpIfTrue(i64, usize),
    JmpIfFalse(i64, usize),
    LessThan(i64, i64, usize),
    Equals(i64, i64, usize),
}

#[derive(Clone, Copy)]
enum ParamMode {
    Pos,
    Imm,
}

fn mode(modes: i64, i: i64) -> ParamMode {
    match (modes / 10_i64.pow(i as u32)) % 10 {
        0 => ParamMode::Pos,
        1 => ParamMode::Imm,
        _ => unreachable!(),
    }
}

impl IntcodeMachine {
    fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            input: VecDeque::new(),
            ip: 0,
            halted: false,
        }
    }

    #[inline]
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    fn get_param(&self, param_kind: ParamMode, val: i64) -> i64 {
        match param_kind {
            ParamMode::Pos => self.memory[val as usize],
            ParamMode::Imm => val,
        }
    }

    fn parse_op(&self) -> Operation {
        let instr = self.memory[self.ip];
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            99 => Operation::Halt,
            1 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]);
                let p3 = self.memory[self.ip + 3] as usize;
                Operation::Add(p1, p2, p3)
            }
            2 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]);
                let p3 = self.memory[self.ip + 3] as usize;
                Operation::Mul(p1, p2, p3)
            }
            3 => {
                let p1 = self.memory[self.ip + 1] as usize;
                Operation::In(p1)
            }
            4 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                Operation::Out(p1)
            }
            5 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]) as usize;
                Operation::JmpIfTrue(p1, p2)
            }
            6 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]) as usize;
                Operation::JmpIfFalse(p1, p2)
            }
            7 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]);
                let p3 = self.memory[self.ip + 3] as usize;
                Operation::LessThan(p1, p2, p3)
            }
            8 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[self.ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[self.ip + 2]);
                let p3 = self.memory[self.ip + 3] as usize;
                Operation::Equals(p1, p2, p3)
            }
            _ => panic!("unsupported op: {opcode}"),
        }
    }

    fn run(&mut self) -> Step {
        if self.halted {
            return Step::Halt;
        }
        loop {
            let op = self.parse_op();
            match op {
                Operation::Halt => {
                    self.halted = true;
                    return Step::Halt;
                }
                Operation::Add(p1, p2, p3) => {
                    self.memory[p3] = p1 + p2;
                    self.ip += 4;
                }
                Operation::Mul(p1, p2, p3) => {
                    self.memory[p3] = p1 * p2;
                    self.ip += 4;
                }
                Operation::In(p1) => {
                    match self.input.pop_front() {
                        None => return Step::AwaitingInput,
                        Some(v) => {
                            self.memory[p1] = v;
                        }
                    }
                    self.ip += 2;
                }
                Operation::Out(p1) => {
                    self.ip += 2;
                    return Step::Output(p1);
                }
                Operation::JmpIfTrue(p1, p2) => {
                    if p1 != 0 {
                        self.ip = p2;
                    } else {
                        self.ip += 3;
                    }
                }
                Operation::JmpIfFalse(p1, p2) => {
                    if p1 == 0 {
                        self.ip = p2;
                    } else {
                        self.ip += 3;
                    }
                }
                Operation::LessThan(p1, p2, p3) => {
                    self.memory[p3] = if p1 < p2 { 1 } else { 0 };
                    self.ip += 4;
                }
                Operation::Equals(p1, p2, p3) => {
                    self.memory[p3] = if p1 == p2 { 1 } else { 0 };
                    self.ip += 4;
                }
            }
        }
    }
}

fn part_one(program: &[i64]) -> i64 {
    let mut best = i64::MIN;
    for p in [0, 1, 2, 3, 4].into_iter().permutations(5) {
        let mut signal = 0;
        for phase in p {
            let mut machine = IntcodeMachine::new(program.to_owned());
            machine.input.push_back(phase);
            machine.input.push_back(signal);
            match machine.run() {
                Step::Output(v) => signal = v,
                _ => unimplemented!(),
            }
        }
        best = best.max(signal);
    }
    best
}

fn part_two(program: &[i64]) -> i64 {
    let mut best = i64::MIN;
    for p in [5, 6, 7, 8, 9].into_iter().permutations(5) {
        let mut machines: Vec<_> = (0..5)
            .map(|i| {
                let mut machine = IntcodeMachine::new(program.to_owned());
                machine.input.push_back(p[i]);
                if i == 0 {
                    machine.input.push_back(0);
                }
                machine
            })
            .collect();

        let mut i = 0;
        while !machines.iter().all(|m| m.is_halted()) {
            loop {
                match machines[i].run() {
                    Step::Halt | Step::AwaitingInput => break,
                    Step::Output(v) => machines[(i + 1) % 5].input.push_back(v),
                }
            }
            i = (i + 1) % 5;
        }
        best = best.max(*machines[0].input.iter().last().unwrap());
    }
    best
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let opcodes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", part_one(&opcodes));
    println!("{}", part_two(&opcodes));
}
