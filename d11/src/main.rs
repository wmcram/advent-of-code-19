use std::collections::{HashSet, VecDeque};

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
    relative_base: usize,
}

const MEM_SIZE: usize = 32000;
impl IntcodeMachine {
    pub fn new(mut memory: Vec<i64>) -> Self {
        memory.resize(MEM_SIZE, 0);
        Self {
            memory,
            input: VecDeque::new(),
            ip: 0,
            halted: false,
            relative_base: 0,
        }
    }

    #[inline]
    pub fn is_halted(&self) -> bool {
        self.halted
    }

    #[inline]
    pub fn feed(&mut self, v: i64) {
        self.input.push_back(v);
    }

    fn get_param(&self, idx: usize) -> i64 {
        let imode = self.memory[self.ip] / (10 * 10_i64.pow(idx as u32)) % 10;
        let val = self.memory[self.ip + idx];

        match imode {
            0 => self.memory[val as usize],
            1 => val,
            2 => self.memory[(val + self.relative_base as i64) as usize],
            _ => unimplemented!(),
        }
    }

    fn set_param(&mut self, idx: usize, val: i64) {
        let imode = self.memory[self.ip] / (10 * 10_i64.pow(idx as u32)) % 10;
        let loc = self.memory[self.ip + idx];
        match imode {
            0 => self.memory[loc as usize] = val,
            2 => self.memory[(loc + self.relative_base as i64) as usize] = val,
            _ => unimplemented!(),
        }
    }

    fn run(&mut self) -> Step {
        if self.halted {
            return Step::Halt;
        }
        loop {
            let instr = self.memory[self.ip] % 100;
            match instr {
                99 => {
                    self.halted = true;
                    return Step::Halt;
                }
                1 => {
                    self.set_param(3, self.get_param(1) + self.get_param(2));
                    self.ip += 4;
                }
                2 => {
                    self.set_param(3, self.get_param(1) * self.get_param(2));
                    self.ip += 4;
                }
                3 => {
                    match self.input.pop_front() {
                        None => return Step::AwaitingInput,
                        Some(v) => {
                            self.set_param(1, v);
                        }
                    }
                    self.ip += 2;
                }
                4 => {
                    let out = self.get_param(1);
                    self.ip += 2;
                    return Step::Output(out);
                }
                5 => {
                    if self.get_param(1) != 0 {
                        self.ip = self.get_param(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    if self.get_param(1) == 0 {
                        self.ip = self.get_param(2) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    self.set_param(
                        3,
                        if self.get_param(1) < self.get_param(2) {
                            1
                        } else {
                            0
                        },
                    );
                    self.ip += 4;
                }
                8 => {
                    self.set_param(
                        3,
                        if self.get_param(1) == self.get_param(2) {
                            1
                        } else {
                            0
                        },
                    );
                    self.ip += 4;
                }
                9 => {
                    self.relative_base = (self.relative_base as i64 + self.get_param(1)) as usize;
                    self.ip += 2;
                }
                _ => unimplemented!(),
            }
        }
    }
}

fn part_one(program: &[i64]) {
    let mut all_time = HashSet::new();
    let mut white = HashSet::new();
    let mut machine = IntcodeMachine::new(program.to_owned());
    let mut pos = (0, 0);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut diri = 0;
    while !machine.is_halted() {
        match machine.run() {
            Step::AwaitingInput => {
                let color = if white.contains(&pos) { 1 } else { 0 };
                machine.feed(color);
            }
            Step::Halt => break,
            Step::Output(color) => {
                let turn = match machine.run() {
                    Step::Output(t) => t,
                    _ => panic!("outputs must be back-to-back"),
                };

                // paint
                if color == 1 {
                    white.insert(pos);
                    all_time.insert(pos);
                } else if white.contains(&pos) {
                    white.remove(&pos);
                }

                // turn
                if turn == 1 {
                    diri += 1;
                } else {
                    diri += 3;
                }
                diri %= 4;

                // move
                pos = (pos.0 + dirs[diri].0, pos.1 + dirs[diri].1);
            }
        }
    }
    println!("{}", all_time.len());
}

fn part_two(program: &[i64]) {
    let mut white = HashSet::new();
    let mut machine = IntcodeMachine::new(program.to_owned());
    let mut pos = (0, 0);
    white.insert(pos);
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut diri = 0;
    while !machine.is_halted() {
        match machine.run() {
            Step::AwaitingInput => {
                let color = if white.contains(&pos) { 1 } else { 0 };
                machine.feed(color);
            }
            Step::Halt => break,
            Step::Output(color) => {
                let turn = match machine.run() {
                    Step::Output(t) => t,
                    _ => panic!("outputs must be back-to-back"),
                };

                // paint
                if color == 1 {
                    white.insert(pos);
                } else if white.contains(&pos) {
                    white.remove(&pos);
                }

                // turn
                if turn == 1 {
                    diri += 1;
                } else {
                    diri += 3;
                }
                diri %= 4;

                // move
                pos = (pos.0 + dirs[diri].0, pos.1 + dirs[diri].1);
            }
        }
    }

    let l = white.iter().map(|p| p.0).min().unwrap();
    let r = white.iter().map(|p| p.0).max().unwrap();
    let d = white.iter().map(|p| p.1).min().unwrap();
    let u = white.iter().map(|p| p.1).max().unwrap();
    for y in (d..=u).into_iter().rev() {
        let mut row = String::with_capacity((d..=u).count());
        for x in l..=r {
            row.push(if white.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("{row}");
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let opcodes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    part_one(&opcodes);
    part_two(&opcodes);
}
