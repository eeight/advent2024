use std::{cmp::min, fs::read_to_string};

use itertools::Itertools;

type V = (i32, i32);

struct Input {
    a: V,
    b: V,
    t: V
}

fn parse() -> Vec<Input>
{
    let input_text = read_to_string("13.txt").unwrap();

    let parse_line = |l: &str| {
        let l = l.split_once(": ").unwrap().1;
        let (x, y) = l.split_once(", ").unwrap();
        (x[2..].parse::<i32>().unwrap(), y[2..].parse::<i32>().unwrap())
    };

    input_text.lines()
        .filter(|l| l.len() > 1)
        .map(parse_line)
        .tuples()
        .map(|(a, b, t)| Input {a, b, t})
        .collect()
}

fn times(v: V, k: i32) -> V {
    (v.0 * k, v.1 * k)
}

fn sub(x: V, y: V) -> V {
    (x.0 - y.0, x.1 - y.1)
}

fn solve(input: &Input) -> Option<i32>
{
    let mut best = 1000;

    for i in 0..101 {
        let t = sub(input.t, times(input.a, i));
        let j = t.0 / input.b.0;

        if sub(t, times(input.b, j)) == (0, 0) && j <= 100 {
            best = min(best, i * 3 + j);
        }
    }

    if best != 1000 { Some(best) } else { None }
}


fn main()
{
    let result: i32 = parse().iter().filter_map(|x| solve(x)).sum();
    println!("{}", result);
}