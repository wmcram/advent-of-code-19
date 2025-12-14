struct IntcodeMachine {
    memory: Vec<u64>,
}

impl IntcodeMachine {
    fn run(&mut self) -> Option<u64> {
        for i in (0..self.memory.len()).step_by(4) {
            match self.memory[i] {
                99 => break,
                1 | 2 => {
                    let in1 = *self.memory.get(i + 1)? as usize;
                    let in2 = *self.memory.get(i + 2)? as usize;
                    let out = *self.memory.get(i + 3)? as usize;
                    if self.memory[i] == 1 {
                        *self.memory.get_mut(out)? =
                            self.memory.get(in1)? + self.memory.get(in2)?;
                    } else {
                        *self.memory.get_mut(out)? =
                            self.memory.get(in1)? * self.memory.get(in2)?;
                    }
                }
                _ => return None,
            }
        }

        Some(self.memory[0])
    }
}

impl From<Vec<u64>> for IntcodeMachine {
    fn from(value: Vec<u64>) -> Self {
        Self { memory: value }
    }
}

fn part_one(opcodes: &[u64]) -> u64 {
    let mut machine = IntcodeMachine::from(opcodes.to_owned());
    machine.memory[1] = 12;
    machine.memory[2] = 2;
    machine.run().unwrap()
}

fn part_two(opcodes: &[u64]) -> u64 {
    let target = 19690720;
    // cartesian product for all pairs (i,j)
    let iter = (0u64..).flat_map(|s| (0..=s).map(move |i| (i, s - i)));

    for (i, j) in iter {
        let mut machine = IntcodeMachine::from(opcodes.to_owned());
        machine.memory[1] = i;
        machine.memory[2] = j;
        if let Some(out) = machine.run()
            && out == target
        {
            return 100 * i + j;
        }
    }
    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let opcodes: Vec<u64> = input
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    println!("{}", part_one(&opcodes));
    println!("{}", part_two(&opcodes));
}
