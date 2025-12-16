fn apply_gravity(moons: &mut [Moon]) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            let (l, r) = moons.split_at_mut(j);
            let m1 = &mut l[i];
            let m2 = &mut r[0];
            m1.apply_gravity(m2);
        }
    }
}

fn apply_velocity(moons: &mut [Moon]) {
    moons.iter_mut().for_each(Moon::apply_velocity);
}

fn part_one(input: &[Moon]) {
    let mut moons = input.to_owned();
    for _ in 0..1000 {
        apply_gravity(&mut moons);
        apply_velocity(&mut moons);
    }
    let energy = moons
        .iter()
        .map(|m| {
            (m.pos.0.abs() + m.pos.1.abs() + m.pos.2.abs())
                * (m.vel.0.abs() + m.vel.1.abs() + m.vel.2.abs())
        })
        .sum::<i32>();
    println!("{energy}");
}

#[derive(Clone, PartialEq, Eq)]
struct Axis {
    pos: Vec<i32>,
    vel: Vec<i32>,
}

impl Axis {
    fn step(&mut self) {
        let n = self.pos.len();
        for i in 0..n {
            for j in i + 1..n {
                let d = (self.pos[j] - self.pos[i]).signum();
                self.vel[i] += d;
                self.vel[j] -= d;
            }
        }
        for i in 0..n {
            self.pos[i] += self.vel[i];
        }
    }

    fn find_period(&self) -> u64 {
        let mut state = self.clone();
        let mut t = 0;
        loop {
            state.step();
            t += 1;
            if state == *self {
                return t;
            }
        }
    }
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

const fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part_two(input: &[Moon]) {
    let x_axis = Axis {
        pos: input.iter().map(|m| m.pos.0).collect(),
        vel: vec![0; input.len()],
    };
    let x_period = x_axis.find_period();
    let y_axis = Axis {
        pos: input.iter().map(|m| m.pos.1).collect(),
        vel: vec![0; input.len()],
    };
    let y_period = y_axis.find_period();
    let z_axis = Axis {
        pos: input.iter().map(|m| m.pos.2).collect(),
        vel: vec![0; input.len()],
    };
    let z_period = z_axis.find_period();
    let period = lcm(x_period, lcm(y_period, z_period));
    println!("{period}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Moon {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
}

impl Moon {
    const fn apply_gravity(&mut self, other: &mut Self) {
        let dx = (other.pos.0 - self.pos.0).signum();
        let dy = (other.pos.1 - self.pos.1).signum();
        let dz = (other.pos.2 - self.pos.2).signum();
        self.vel.0 += dx;
        other.vel.0 -= dx;
        self.vel.1 += dy;
        other.vel.1 -= dy;
        self.vel.2 += dz;
        other.vel.2 -= dz;
    }

    const fn apply_velocity(&mut self) {
        let (x, y, z) = self.pos;
        let (dx, dy, dz) = self.vel;
        self.pos = (x + dx, y + dy, z + dz);
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let moons: Vec<_> = input
        .lines()
        .map(|l| {
            let mut coords = l[1..l.len() - 1]
                .split(',')
                .map(|t| t.trim().split_once('=').unwrap().1.parse::<i32>().unwrap());
            Moon {
                pos: (
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                ),
                vel: (0, 0, 0),
            }
        })
        .collect();
    part_one(&moons);
    part_two(&moons);
}
