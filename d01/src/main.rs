fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|l| (l.parse::<u64>().unwrap() / 3) - 2)
        .sum()
}

fn part_two(input: &str) -> u64 {
    let mut res: u64 = 0;
    input.lines().for_each(|l| {
        let mut amt: i32 = l.parse().unwrap();
        loop {
            let next = (amt / 3) - 2;
            if next < 0 {
                break;
            }
            res += next as u64;
            amt = next;
        }
    });
    res
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}
