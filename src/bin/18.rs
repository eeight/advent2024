use std::{collections::HashSet, fs::read_to_string};

use pathfinding::prelude::dijkstra;

type V = (i32, i32);

static SIZE: i32 = 71;
static TAKE_FIRST: usize = 1024;

fn in_range(x: i32) -> bool {
    x >= 0 && x < SIZE
}

fn parse() -> HashSet<V>
{
    let input_text = read_to_string("18.txt").unwrap();
    input_text
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .take(TAKE_FIRST)
        .collect()
}

fn solve(bytes: &HashSet<V>) -> i32 {
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
    
    return dijkstra(&(0, 0), next, success).unwrap().1;
}

fn main()
{
    let bytes = parse();
    println!("{}", solve(&bytes));
}