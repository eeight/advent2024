use std::{collections::HashMap, fs::read_to_string};

use itertools::iterate;

static M: u64 = 16777216;

fn parse() -> Vec<u64>
{
    let input_text = read_to_string("22.txt").unwrap();
    input_text.lines().map(|l| l.parse().unwrap()).collect()
}

fn next(n: &u64) -> u64
{
    let mix = |r, n| (n ^ r) % M;
    let n = mix(n * 64, *n);
    let n = mix(n / 32, n);
    mix(n * 2048, n)
}

fn diff_seq(n: u64) -> (Vec<u64>, Vec<i8>)
{
    let mut ns: Vec<_> = iterate(n, next).take(2000).collect();
    let ds = ns.windows(2).map(|vs| (vs[1] % 10) as i8 - (vs[0] % 10) as i8).collect();
    ns.remove(0);
    (ns, ds)
}

fn index(ds: &[i8]) -> HashMap<u64, usize>
{
    let mut result = HashMap::new();

    for (i, vs) in ds.windows(4).enumerate() {
        let key = vs
            .iter()
            .map(|x| (x + 9) as u64)
            .zip(iterate(1, |x| x * 19))
            .map(|(x, y)| x * y)
            .sum();
        result.entry(key).or_insert(i + 3);
    }

    result
}

fn solve(ns: &[u64]) -> u64
{
    let ds: Vec<_> = ns
        .iter()
        .map(|n| {
            let (ns, ds) = diff_seq(*n);
            (ns, index(&ds))
        })
        .collect();

    let mut result = 0;

    for key in 0..(19*19*19*19) {
        let r: u64 = ds
            .iter()
            .map(|(ns, ix)|
                match ix.get(&key) {
                    Some(i)  => ns[*i] % 10,
                    None => 0
                }
            ).sum();
        if r > result {
            result = r;
        }
    }

    result
}

fn main()
{
    let ns = parse();
    println!("{}", solve(&ns));
}