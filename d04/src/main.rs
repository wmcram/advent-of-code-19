use std::ops::RangeInclusive;

fn part_one(range: RangeInclusive<u64>) -> u64 {
    let mut res = 0;

    for mut n in range {
        let mut prev = n % 10;
        n /= 10;
        let mut dec = true;
        let mut adj = false;
        while n > 0 {
            if n % 10 > prev {
                dec = false;
                break;
            } else if n % 10 == prev {
                adj = true;
            }
            prev = n % 10;
            n /= 10;
        }
        if dec && adj {
            res += 1;
        }
    }

    res
}

fn part_two(range: RangeInclusive<u64>) -> u64 {
    let mut res = 0;

    for mut n in range {
        let mut prev = n % 10;
        n /= 10;
        let mut dec = true;
        let mut adj = false;
        let mut cur = 1;
        while n > 0 {
            if n % 10 > prev {
                dec = false;
                break;
            } else if n % 10 == prev {
                cur += 1;
            } else {
                if cur == 2 {
                    adj = true;
                }
                cur = 1;
            }
            prev = n % 10;
            n /= 10;
        }
        if cur == 2 {
            adj = true;
        }
        if dec && adj {
            res += 1;
        }
    }

    res
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let (start, end) = input.trim().split_once('-').unwrap();
    let range: RangeInclusive<u64> =
        RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap());
    println!("{}", part_one(range.clone()));
    println!("{}", part_two(range));
}
