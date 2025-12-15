use std::collections::{HashMap, HashSet};

fn get_path<'a>(mut from: &'a str, adj: &'a HashMap<&str, &str>) -> Vec<&'a str> {
    let mut res = vec![from];
    while adj.contains_key(from) {
        from = adj[from];
        res.push(from);
    }
    res
}

fn part_one(adj: &HashMap<&str, &str>) -> u64 {
    adj.keys()
        .map(|p| (get_path(p, adj).len() - 1) as u64)
        .sum()
}

fn part_two(adj: &HashMap<&str, &str>) -> u64 {
    let you_path = get_path("YOU", adj);
    let san_path = get_path("SAN", adj);
    let inter: HashSet<_> =
        &you_path.iter().copied().collect::<HashSet<_>>() & &san_path.iter().copied().collect();
    let you_dist = you_path
        .iter()
        .enumerate()
        .find(|&(_, p)| inter.contains(p))
        .unwrap()
        .0;
    let san_dist = san_path
        .iter()
        .enumerate()
        .find(|&(_, p)| inter.contains(p))
        .unwrap()
        .0;
    (you_dist + san_dist - 2) as u64
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let edges: Vec<_> = input
        .lines()
        .map(|l| l.split_once(')').unwrap())
        .map(|(x, y)| (y, x))
        .collect();
    let mut adj: HashMap<&str, &str> = HashMap::new();
    for (s, e) in edges {
        adj.insert(s, e);
    }
    println!("{}", part_one(&adj));
    println!("{}", part_two(&adj));
}
