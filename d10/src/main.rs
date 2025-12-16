use std::collections::{HashMap, HashSet};

fn gcd(mut a: i32, mut b: i32) -> i32 {
    if a <= b {
        std::mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn part_one(input: &str) {
    let mut asts = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asts.insert((x as i32, y as i32));
            }
        }
    }

    let mut best = 0;
    let mut best_coords = (0, 0);
    for &(cx, cy) in &asts {
        let mut dirs = HashSet::new();
        for &(x, y) in &asts {
            let (dx, dy) = (x - cx, y - cy);
            if dx == 0 && dy == 0 {
                continue;
            }
            let g = gcd(dx.abs(), dy.abs());
            dirs.insert((dx / g, dy / g));
        }
        if dirs.len() > best {
            best = dirs.len();
            best_coords = (cx, cy);
        }
    }

    println!("{best}");
    println!("{best_coords:?}");
}

fn angle((dx, dy): (i32, i32)) -> f64 {
    let a = (dx as f64).atan2(-dy as f64);
    if a < 0.0 {
        a + 2.0 * std::f64::consts::PI
    } else {
        a
    }
}

fn part_two(input: &str) {
    let mut asts = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asts.insert((x as i32, y as i32));
            }
        }
    }

    let (cx, cy) = (11, 13);

    let mut by_dir: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    for &(x, y) in &asts {
        let dx = x - cx;
        let dy = y - cy;
        if dx == 0 && dy == 0 {
            continue;
        }
        let g = gcd(dx.abs(), dy.abs());
        by_dir.entry((dx / g, dy / g)).or_default().push((x, y));
    }

    for v in by_dir.values_mut() {
        v.sort_by_key(|&(x, y)| {
            let dx = x - cx;
            let dy = y - cy;
            dx * dx + dy * dy
        });
    }

    let mut dirs: Vec<_> = by_dir.into_iter().collect();
    dirs.sort_by(|a, b| angle(a.0).partial_cmp(&angle(b.0)).unwrap());

    let mut vaporized = 0;
    loop {
        for (_, asts) in dirs.iter_mut() {
            if let Some((x, y)) = asts.first().cloned() {
                asts.remove(0);
                vaporized += 1;
                if vaporized == 200 {
                    println!("{}", x * 100 + y);
                    return;
                }
            }
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    part_one(&input);
    part_two(&input);
}
