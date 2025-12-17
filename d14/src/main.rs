#![allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
use std::collections::HashMap;

type ReactionMap<'a> = HashMap<&'a str, (u64, Vec<(&'a str, u64)>)>;

fn ore_for_fuel(reactions: &ReactionMap, fuel: u64) -> u64 {
    let mut need = HashMap::new();
    need.insert("FUEL", fuel as i64);
    let mut excess = HashMap::new();

    while let Some((&chem, &amt)) = need.iter().find(|&(c, a)| *c != "ORE" && *a > 0) {
        need.remove(chem);
        let have = excess.remove(chem).unwrap_or(0);
        let amt = amt - have;
        if amt <= 0 {
            excess.insert(chem, -amt);
            continue;
        }

        let (out_amt, inputs) = reactions.get(chem).unwrap();
        let runs = u64::div_ceil(amt as u64, *out_amt);
        let produced = runs * out_amt;

        if produced as i64 > amt {
            excess.insert(chem, produced as i64 - amt);
        }

        for (ing, cost) in inputs {
            *need.entry(ing).or_insert(0) += (runs * cost) as i64;
        }
    }
    need["ORE"] as u64
}

fn part_one(reactions: &ReactionMap) {
    println!("{}", ore_for_fuel(reactions, 1));
}

fn part_two(reactions: &ReactionMap) {
    let ore = 1_000_000_000_000_u64;
    let (mut l, mut r) = (1, 1);
    while ore_for_fuel(reactions, r) <= ore {
        r *= 2;
    }
    while l < r {
        let m = (l + r).div_ceil(2);
        if ore_for_fuel(reactions, m) <= ore {
            l = m;
        } else {
            r = m - 1;
        }
    }
    println!("{l}");
}

fn parse_chem(s: &str) -> (&str, u64) {
    let (amt, name) = s.trim().split_once(' ').unwrap();
    (name, amt.parse().unwrap())
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let reactions: ReactionMap = input
        .lines()
        .map(|l| {
            let (inputs, product) = l.split_once("=>").unwrap();
            let (prod_name, prod_amt) = parse_chem(product);
            let costs = inputs.split(',').map(parse_chem).collect();
            (prod_name, (prod_amt, costs))
        })
        .collect();
    part_one(&reactions);
    part_two(&reactions);
}
