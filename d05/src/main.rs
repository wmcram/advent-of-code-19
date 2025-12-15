use std::iter::repeat_n;

struct IntcodeMachine {
    memory: Vec<i64>,
    input: i64,
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
    fn new(memory: Vec<i64>, input: i64) -> Self {
        Self { memory, input }
    }

    fn get_param(&self, param_kind: ParamMode, val: i64) -> i64 {
        match param_kind {
            ParamMode::Pos => self.memory[val as usize],
            ParamMode::Imm => val,
        }
    }

    fn parse_op(&self, ip: usize) -> Operation {
        let instr = self.memory[ip];
        let opcode = instr % 100;
        let modes = instr / 100;
        match opcode {
            99 => Operation::Halt,
            1 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]);
                let p3 = self.memory[ip + 3] as usize;
                Operation::Add(p1, p2, p3)
            }
            2 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]);
                let p3 = self.memory[ip + 3] as usize;
                Operation::Mul(p1, p2, p3)
            }
            3 => {
                let p1 = self.memory[ip + 1] as usize;
                Operation::In(p1)
            }
            4 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                Operation::Out(p1)
            }
            5 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]) as usize;
                Operation::JmpIfTrue(p1, p2)
            }
            6 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]) as usize;
                Operation::JmpIfFalse(p1, p2)
            }
            7 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]);
                let p3 = self.memory[ip + 3] as usize;
                Operation::LessThan(p1, p2, p3)
            }
            8 => {
                let p1 = self.get_param(mode(modes, 0), self.memory[ip + 1]);
                let p2 = self.get_param(mode(modes, 1), self.memory[ip + 2]);
                let p3 = self.memory[ip + 3] as usize;
                Operation::Equals(p1, p2, p3)
            }
            _ => panic!("unsupported op: {opcode}"),
        }
    }

    fn run(mut self) -> Vec<i64> {
        let mut ip = 0;
        let mut output = Vec::new();
        loop {
            let op = self.parse_op(ip);
            match op {
                Operation::Halt => break,
                Operation::Add(p1, p2, p3) => {
                    self.memory[p3] = p1 + p2;
                    ip += 4;
                }
                Operation::Mul(p1, p2, p3) => {
                    self.memory[p3] = p1 * p2;
                    ip += 4;
                }
                Operation::In(p1) => {
                    self.memory[p1] = self.input;
                    ip += 2;
                }
                Operation::Out(p1) => {
                    output.push(p1);
                    ip += 2;
                }
                Operation::JmpIfTrue(p1, p2) => {
                    if p1 != 0 {
                        ip = p2;
                    } else {
                        ip += 3;
                    }
                }
                Operation::JmpIfFalse(p1, p2) => {
                    if p1 == 0 {
                        ip = p2;
                    } else {
                        ip += 3;
                    }
                }
                Operation::LessThan(p1, p2, p3) => {
                    self.memory[p3] = if p1 < p2 { 1 } else { 0 };
                    ip += 4;
                }
                Operation::Equals(p1, p2, p3) => {
                    self.memory[p3] = if p1 == p2 { 1 } else { 0 };
                    ip += 4;
                }
            }
        }
        output
    }
}

fn part_one(program: &[i64]) -> i64 {
    let machine = IntcodeMachine::new(program.to_owned(), 1);
    *machine.run().last().unwrap()
}

fn part_two(program: &[i64]) -> i64 {
    let machine = IntcodeMachine::new(program.to_owned(), 5);
    *machine.run().first().unwrap()
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
