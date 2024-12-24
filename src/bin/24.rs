use std::{collections::{HashMap, HashSet, VecDeque}, fs::read_to_string};

use itertools::iterate;

enum Op
{
    And,
    Or,
    Xor
}

fn parse() -> (Vec<(String, bool)>, Vec<(String, String, String, Op)>)
{
    let mut wires = Vec::new();
    let mut ops = Vec::new();
    let input_text = read_to_string("24.txt").unwrap();

    let mut iter = input_text.lines();

    loop {
        let l = iter.next().unwrap();
        if l.is_empty() {
            break;
        }
        let (name, v) = l.split_once(": ").unwrap();
        wires.push((name.to_string(), v.parse::<i32>().unwrap() != 0));
    }

    for l in iter {
        let ps: Vec<_> = l.split(' ').collect();
        let op = match ps[1] {
            "XOR" => Op::Xor,
            "AND" => Op::And,
            "OR" => Op::Or,
            _ => unreachable!()
        };
        ops.push((ps[0].to_string(), ps[2].to_string(), ps[4].to_string(), op));
    }

    (wires, ops)
}

fn solve(wires: &[(String, bool)], ops: &[(String, String, String, Op)]) -> u64
{
    let mut q: VecDeque<(&str, bool)> = wires.iter().map(|(n, v)| (n.as_str(), *v)).collect();
    let mut known_wires: HashMap<&str, bool> = HashMap::new();
    let mut unknown_zs: HashSet<&str> = HashSet::new();

    for (a, b, c, _) in ops {
        for x in &[a, b, c] {
            if x.starts_with('z') {
                unknown_zs.insert(x);
            }
        }
    }

    while !unknown_zs.is_empty() {
        assert!(!q.is_empty());
        let (name, v) = q.pop_front().unwrap();
        known_wires.insert(name, v);
        unknown_zs.remove(name);

        for (a, b, c, op) in ops {
            if known_wires.contains_key(c.as_str()) {
                continue;
            }

            if let (Some(a), Some(b)) = (known_wires.get(a.as_str()), known_wires.get(b.as_str())) {
                let res = match op {
                    Op::And => *a && *b,
                    Op::Xor => *a != *b,
                    Op::Or => *a || *b
                };
                known_wires.insert(&c, res);
                q.push_back((c, res));
            }
        }
    }

    let mut z_wires: Vec<(u32, bool)> = known_wires
        .iter()
        .filter_map(|(k, v)|
            Some((k.strip_prefix('z')?.parse::<u32>().unwrap(), *v))
        )
        .collect();

    z_wires.sort();

    z_wires
        .iter()
        .zip(iterate(1, |x| x * 2))
        .map(|((_, v), p)| *v as u64 * p)
        .sum()
}

fn main()
{
    let (ws, ops) = parse();
    println!("{}", solve(&ws, &ops));
}