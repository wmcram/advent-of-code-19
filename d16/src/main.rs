use std::iter::{repeat_n, zip};

fn gen_pattern(idx: usize) -> impl Iterator<Item = i32> {
    let n = idx + 1;
    repeat_n(0, n)
        .chain(repeat_n(1, n))
        .chain(repeat_n(0, n))
        .chain(repeat_n(-1, n))
        .cycle()
        .skip(1)
}

fn fft_phase(input: &Vec<u32>) -> Vec<u32> {
    let mut out = Vec::with_capacity(input.len());
    for i in 0..input.len() {
        let pat = gen_pattern(i);
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
        let val: u32 = (zip(input, pat)
            .map(|(&x, p)| (x as i32 * p))
            .sum::<i32>()
            .abs()
            % 10) as u32;
        out.push(val);
    }
    out
}

fn part_one(digs: &[u32]) {
    let mut digs = digs.to_owned();
    for _ in 0..100 {
        digs = fft_phase(&digs);
    }
    println!("{:?}", digs.iter().take(8).collect::<Vec<_>>());
}

fn part_two(digs: &[u32]) {
    let msg_offset = digs.iter().take(7).fold(0, |acc, &v| acc * 10 + v as usize);
    let msg_end = digs.len() * 10_000;

    // build only part we need
    let mut cur = Vec::new();
    for i in msg_offset..msg_end {
        cur.push(digs[i % digs.len()]);
    }

    for _ in 0..100 {
        // make prefix sums
        let mut pres = vec![0];
        let mut tot = 0;
        for &i in &cur {
            tot += i;
            pres.push(tot);
        }

        for i in 0..cur.len() {
            // suffix total
            let v = pres.last().unwrap() - pres[i];
            cur[i] = v % 10;
        }
    }

    let res = cur.iter().take(8).fold(0, |acc, &v| acc * 10 + v as usize);
    println!("{res}");
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let digs: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    part_one(&digs);
    part_two(&digs);
}
