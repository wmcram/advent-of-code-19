use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
pub enum Step {
    AwaitingInput,
    Output(i64),
    Halt,
}

pub struct IntcodeMachine {
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
    #[allow(unused)]
    pub fn feed(&mut self, v: i64) {
        self.input.push_back(v);
    }

    #[allow(unused)]
    pub fn feed_many(&mut self, vs: &[i64]) {
        self.input.extend(vs);
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

    pub fn run(&mut self) -> Step {
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
                    self.set_param(3, i64::from(self.get_param(1) < self.get_param(2)));
                    self.ip += 4;
                }
                8 => {
                    self.set_param(3, i64::from(self.get_param(1) == self.get_param(2)));
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

    pub fn run_to_await(&mut self) -> Vec<i64> {
        let mut out = Vec::new();
        while !self.is_halted() {
            match self.run() {
                Step::AwaitingInput | Step::Halt => break,
                Step::Output(v) => out.push(v),
            }
        }
        out
    }
}
