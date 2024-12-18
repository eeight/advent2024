use std::{collections::HashSet, fs::read_to_string};

use pathfinding::prelude::dijkstra;

type V = (i32, i32);

static SIZE: i32 = 71;

fn in_range(x: i32) -> bool {
    x >= 0 && x < SIZE
}

fn parse() -> Vec<V>
{
    let input_text = read_to_string("18.txt").unwrap();
    input_text
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

fn has_path(bytes: &[V]) -> bool {
    let bytes: HashSet<V> = bytes.iter().copied().collect();
    let next = |(x, y): &V| -> Vec<(V, i32)> {
        let mut ns = Vec::new();
            for a in &[-1, 1] {
                for b in &[-1, 1] {
                    let x1 = x + (a + b) / 2;
                    let y1 = y + (a - b) / 2;
                    if in_range(x1) && in_range(y1) && !bytes.contains(&(x1, y1)) {
                        ns.push(((x1, y1), 1));
                    }
                }
            }
        ns
    };

    let success = |v: &V| *v == (SIZE - 1, SIZE - 1);
    
    return dijkstra(&(0, 0), next, success).is_some();
}

fn solve(bytes: &[V]) -> V {
    let mut good = 0;
    let mut bad = bytes.len();

    while good + 1 < bad {
        let mid = (good + bad) / 2;
        if has_path(&bytes[..mid]) {
            good = mid;
        } else {
            bad = mid;
        }
    }

    bytes[bad - 1]
}

fn main()
{
    let bytes = parse();
    let (x, y) = solve(&bytes);
    println!("{},{}", x, y);
}