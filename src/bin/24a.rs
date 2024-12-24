use std::{collections::HashMap, fs::read_to_string};

#[derive(PartialEq, Eq, Debug)]
enum Op
{
    And,
    Or,
    Xor
}

fn swaps() -> Vec<(&'static str, &'static str)>
{
    vec![
        ("drg", "z22"),
        ("jbp", "z35"),
        ("jgc", "z15"),
        ("qjb", "gvw"),
    ]
}

fn parse() -> Vec<(String, String, String, Op)>
{
    let mut ops = Vec::new();
    let input_text = read_to_string("24.txt").unwrap();

    let swaps: HashMap<&str, &str> = swaps().iter().flat_map(|(x, y)| [(*x, *y), (*y, *x)]).collect();
    for l in input_text.lines().skip_while(|l| !l.is_empty()).skip(1) {
        let ps: Vec<_> = l.split(' ').collect();
        let op = match ps[1] {
            "XOR" => Op::Xor,
            "AND" => Op::And,
            "OR" => Op::Or,
            _ => unreachable!()
        };
        let target = swaps.get(ps[4]).unwrap_or(&ps[4]);
        ops.push((ps[0].to_string(), ps[2].to_string(), target.to_string(), op));
    }

    ops
}

fn solve(ops: &[(String, String, String, Op)]) -> u64
{
    let mut xors: HashMap<u32, &str> = HashMap::new();
    let mut ands: HashMap<u32, &str> = HashMap::new();

    for (a, _, c, op) in ops {
        if !a.starts_with('x') && !a.starts_with('y') {
            continue;
        }
        let n = a.strip_prefix("x").or(a.strip_prefix("y")).unwrap().parse::<u32>().unwrap();
        match op {
            Op::Xor => { xors.insert(n, c); },
            Op::And => { ands.insert(n, c); },
            _ => unreachable!()
        }
    }
    assert!(ands.len() == 45);
    assert!(xors.len() == 45);

    for (n, wire) in xors {
        for (a, b, c, op) in ops {
            if *op != Op::Xor || (a != wire && b != wire) {
                continue;
            }
            if *c != format!("z{:02}", n) {
                println!("XOR: {} has to be z{:02}", c, n);
            }
        }
    }

    'a: for (n, wire) in ands {
        for (a, b, c, op) in ops {
            if *op == Op::Or && (a == wire || b == wire) {
                println!("??? carry {} is {}", n, c);
                continue 'a;
            }
        }
        println!("Where's carry for {}?", n);
    }
    0
}

fn main()
{
    let ops = parse();
    solve(&ops);
    let mut swaps: Vec<_> = swaps().iter().flat_map(|(x, y)| [*x, *y]).collect();
    swaps.sort();
    println!("{}", swaps.join(","));
}