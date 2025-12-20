mod intcode;

use std::collections::HashSet;

use intcode::IntcodeMachine;

fn right(dir: &(i32, i32)) -> (i32, i32) {
    (dir.1, -dir.0)
}

fn left(dir: &(i32, i32)) -> (i32, i32) {
    right(&right(&right(dir)))
}

fn part_one(program: &[i64]) {
    let mut machine = IntcodeMachine::new(program.to_owned());
    let output: String = machine
        .run_to_await()
        .iter()
        .map(|&i| i as u8 as char)
        .collect();
    let board: Vec<Vec<char>> = output.trim().lines().map(|l| l.chars().collect()).collect();
    println!("{output}");
    let (m, n) = (board.len() as i32, board[0].len() as i32);
    let mut res = 0;
    let dirs = vec![(0, 1), (1, 0), (-1, 0), (0, -1)];

    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if board[i][j] == '^' {
                println!("initial pos: {},{}", i, j);
            }
            if board[i][j] != '#' {
                continue;
            }
            let mut cross = true;
            for (di, dj) in &dirs {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni < 0 || ni == m || nj < 0 || nj == n || board[ni as usize][nj as usize] != '#'
                {
                    cross = false;
                    break;
                }
            }
            if cross {
                res += i * j;
            }
        }
    }
    println!("{res}");

    // this isnt necessary for part one, but I already have the board here so
    // we might as well write out the path
    let mut pos = (10, 36);
    let mut dir = (-1, 0);
    let mut out: Vec<char> = Vec::new();

    // attempts to move pos in the given dir. returns true if pos was moved successfully.
    let mut try_advance = |(di, dj): (i32, i32)| {
        let (ni, nj) = (pos.0 + di, pos.1 + dj);
        if ni >= 0 && nj >= 0 && ni < m && nj < n && board[ni as usize][nj as usize] == '#' {
            pos = (ni, nj);
            return true;
        }
        false
    };

    let mut cur = 0;

    loop {
        while try_advance(dir) {
            cur += 1;
        }
        if cur > 0 {
            out.extend(cur.to_string().chars());
            out.push(',');
        }
        // otherwise check our turns
        if try_advance(left(&dir)) {
            cur = 1;
            out.push('L');
            out.push(',');
            dir = left(&dir);
            continue;
        } else if try_advance(right(&dir)) {
            cur = 1;
            out.push('R');
            out.push(',');
            dir = right(&dir);
            continue;
        }
        // otherwise we must be done.
        break;
    }

    let out: String = out.iter().collect();
    println!("{out}");
}

fn part_two(program: &[i64]) {
    let mut new_prog = program.to_owned();
    new_prog[0] = 2;
    let mut machine = IntcodeMachine::new(new_prog);
    let a = b"R,6,L,10,R,8\n";
    let b = b"R,8,R,12,L,8,L,8\n";
    let c = b"L,10,R,6,R,6,L,8\n";
    let prog = b"A,B,A,B,C,A,B,C,A,C\n";
    machine.feed_many(&prog.iter().map(|&v| i64::from(v)).collect::<Vec<_>>());
    machine.feed_many(&a.iter().map(|&v| i64::from(v)).collect::<Vec<_>>());
    machine.feed_many(&b.iter().map(|&v| i64::from(v)).collect::<Vec<_>>());
    machine.feed_many(&c.iter().map(|&v| i64::from(v)).collect::<Vec<_>>());
    machine.feed_many(&b"n\n".iter().map(|&v| i64::from(v)).collect::<Vec<_>>());

    let out = machine.run_to_await();
    let res = out.last().unwrap();
    println!("{res}");
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
