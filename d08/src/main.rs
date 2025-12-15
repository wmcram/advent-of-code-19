use itertools::Itertools;

const ROWS: usize = 6;
const COLS: usize = 25;
const LAYER_SIZE: usize = ROWS * COLS;

fn part_one(input: &str) -> u64 {
    let mut best_zeroes = usize::MAX;
    let mut res = 0;

    for chunk in input.trim().chars().chunks(LAYER_SIZE).into_iter() {
        let counts = chunk.counts();
        let zeroes = *counts.get(&'0').unwrap_or(&0);

        if zeroes < best_zeroes {
            best_zeroes = zeroes;
            let ones = *counts.get(&'1').unwrap_or(&0);
            let twos = *counts.get(&'2').unwrap_or(&0);
            res = (ones * twos) as u64;
        }
    }

    res
}

fn part_two(input: &str) {
    let mut img = [[2; COLS]; ROWS];
    for layer in input.trim().chars().chunks(LAYER_SIZE).into_iter() {
        for (i, c) in layer.enumerate() {
            if img[i / COLS][i % COLS] == 2 {
                img[i / COLS][i % COLS] = c.to_digit(10).unwrap();
            }
        }
    }
    for row in img {
        for c in row {
            print!(
                "{}",
                match c {
                    1 => '#',
                    0 => '.',
                    _ => unimplemented!(),
                }
            );
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_one(&input));
    part_two(&input);
}
