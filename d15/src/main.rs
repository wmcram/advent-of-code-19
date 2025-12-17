use std::collections::{HashMap, HashSet, VecDeque};

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
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn run_to_await(&mut self) -> Vec<i64> {
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

#[derive(Clone, Copy)]
enum Dir {
    N = 1,
    S = 2,
    W = 3,
    E = 4,
}

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::N => (0, 1),
            Dir::S => (0, -1),
            Dir::W => (-1, 0),
            Dir::E => (1, 0),
        }
    }

    fn rev(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::W => Dir::E,
            Dir::E => Dir::W,
        }
    }
}

fn try_move(machine: &mut IntcodeMachine, dir: Dir) -> i64 {
    machine.feed(dir as i64);
    match machine.run() {
        Step::Output(v) => v,
        _ => unimplemented!(),
    }
}

fn dfs(
    machine: &mut IntcodeMachine,
    pos: (i32, i32),
    dist: u32,
    best: &mut Option<u32>,
    maze: &mut HashMap<(i32, i32), u32>,
) {
    for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
        let (dx, dy) = dir.delta();
        let next = (pos.0 + dx, pos.1 + dy);

        if maze.contains_key(&next) {
            continue;
        }

        match try_move(machine, dir) {
            0 => {
                maze.insert(next, 0);
            }
            x @ (1 | 2) => {
                maze.insert(next, x as u32);
                if x == 2 {
                    *best = Some(dist + 1);
                }
                dfs(machine, next, dist + 1, best, maze);
                // this should always succeed
                try_move(machine, dir.rev());
            }
            _ => unreachable!(),
        }
    }
}

fn part_one(program: &[i64]) {
    let mut machine = IntcodeMachine::new(program.to_owned());
    let mut best = None;
    let mut maze = HashMap::new();
    maze.insert((0, 0), 1);
    dfs(&mut machine, (0, 0), 0, &mut best, &mut maze);
    println!("{best:?}");
    part_two(&maze);
}

fn part_two(maze: &HashMap<(i32, i32), u32>) {
    let (&start_pos, _) = maze.iter().find(|&(_, &v)| v == 2).unwrap();
    let mut q = VecDeque::new();
    let mut seen = HashSet::new();
    let mut ans = 0;

    q.push_back((start_pos, 0));
    seen.insert(start_pos);

    while let Some(((x, y), t)) = q.pop_front() {
        ans = ans.max(t);
        for dir in [Dir::N, Dir::S, Dir::W, Dir::E] {
            let (dx, dy) = dir.delta();
            let next = (x + dx, y + dy);
            if let Some(&status) = maze.get(&next) {
                if status != 0 && !seen.contains(&next) {
                    seen.insert(next);
                    q.push_back((next, t + 1));
                }
            }
        }
    }

    println!("{ans}");
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let opcodes: Vec<i64> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    part_one(&opcodes);
}
