use std::collections::HashMap;

fn parse_dir(c: char) -> (i32, i32) {
    match c {
        'R' => (1, 0),
        'L' => (-1, 0),
        'U' => (0, 1),
        'D' => (0, -1),
        _ => unimplemented!(),
    }
}

fn part_one(wires1: &[&str], wires2: &[&str]) -> u64 {
    let mut points: HashMap<(i32, i32), u32> = HashMap::new();
    let mut pos = (0, 0);
    for w in wires1 {
        let amt: i32 = w[1..].parse().unwrap();
        let dir = parse_dir(w.chars().nth(0).unwrap());
        for _ in 0..amt {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            *points.entry(pos).or_default() |= 1;
        }
    }
    let mut pos = (0, 0);
    for w in wires2 {
        let amt: i32 = w[1..].parse().unwrap();
        let dir = parse_dir(w.chars().nth(0).unwrap());
        for _ in 0..amt {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            *points.entry(pos).or_default() |= 2;
        }
    }

    points
        .iter()
        .filter(|(_, c)| **c == 3)
        .map(|((x, y), _)| (x.abs() + y.abs()) as u64)
        .min()
        .unwrap()
}

fn part_two(wires1: &[&str], wires2: &[&str]) -> u64 {
    let mut points: HashMap<(i32, i32), (u64, u64)> = HashMap::new();
    let mut pos = (0, 0);
    let mut t = 0;
    for w in wires1 {
        let amt: i32 = w[1..].parse().unwrap();
        let dir = parse_dir(w.chars().nth(0).unwrap());
        for _ in 0..amt {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            t += 1;
            let entry = points.entry(pos).or_default();
            if entry.0 == 0 {
                entry.0 = t;
            }
        }
    }
    let mut pos = (0, 0);
    let mut t = 0;
    for w in wires2 {
        let amt: i32 = w[1..].parse().unwrap();
        let dir = parse_dir(w.chars().nth(0).unwrap());
        for _ in 0..amt {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            t += 1;
            let entry = points.entry(pos).or_default();
            if entry.1 == 0 {
                entry.1 = t;
            }
        }
    }

    points
        .values()
        .filter(|(t1, t2)| *t1 > 0 && *t2 > 0)
        .map(|(t1, t2)| (*t1 + *t2))
        .min()
        .unwrap()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut lines = input.lines();
    let wires1: Vec<_> = lines.next().unwrap().trim().split(',').collect();
    let wires2: Vec<_> = lines.next().unwrap().trim().split(',').collect();
    println!("{}", part_one(&wires1, &wires2));
    println!("{}", part_two(&wires1, &wires2));
}
