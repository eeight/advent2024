use std::fs::read_to_string;

static M: u64 = 16777216;

fn parse() -> Vec<u64>
{
    let input_text = read_to_string("22.txt").unwrap();
    input_text.lines().map(|l| l.parse().unwrap()).collect()
}

fn next(n: u64) -> u64
{
    let mix = |r, n| (n ^ r) % M;
    let n = mix(n * 64, n);
    let n = mix(n / 32, n);
    mix(n * 2048, n)
}

fn next_i(mut n: u64, iter: usize) -> u64
{
    for _ in 0..iter {
        n = next(n);
    }
    n
}

fn solve(ns: &[u64]) -> u64
{
    ns.iter().map(|n| next_i(*n, 2000)).sum()
}

fn main()
{
    let ns = parse();
    println!("{}", solve(&ns));
}