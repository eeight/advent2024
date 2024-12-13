use std::{cmp::min, cmp::max, fs::read_to_string};

use itertools::Itertools;

type V = (i64, i64);

#[derive(Debug)]
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
        (x[2..].parse::<i64>().unwrap(), y[2..].parse::<i64>().unwrap())
    };

    input_text.lines()
        .filter(|l| l.len() > 1)
        .map(parse_line)
        .tuples()
        .map(|(a, b, t)| Input {a, b, t:(t.0 + 10000000000000, t.1 + 10000000000000)})
        .collect()
}

fn times(v: V, k: i64) -> V {
    (v.0 * k, v.1 * k)
}

fn add(x: V, y: V) -> V {
    (x.0 + y.0, x.1 + y.1)
}

fn sub(x: V, y: V) -> V {
    (x.0 - y.0, x.1 - y.1)
}

fn solve_small(input: &Input) -> Option<i64>
{
    let mut best = None;

    for i in 0..input.t.0 / input.a.0 {
        let t = sub(input.t, times(input.a, i));
        if t.0 < 0 {
            break;
        }
        let j = t.0 / input.b.0;

        if sub(t, times(input.b, j)) == (0, 0) {
            let cost = i * 3 + j;
            best = Some(min(best.unwrap_or(cost), cost));
        }
    }

    best
}

fn solve(input: &Input) -> Option<i64>
{
    let d = |(x, y)| x - y;
    let da = d(input.a);
    let db = d(input.b);

    if da * db > 0 {
        return None;
    }

    let mut best = None;

    for k in 1..max(da.abs(), db.abs()) + 1 {
        if (da * k).abs() % db.abs() != 0 {
            continue;
        }
        let l = (da * k).abs() / db.abs();
        let u = min(
            input.t.0 / (input.a.0 * k + input.b.0 * l),
            input.t.1 / (input.a.1 * k + input.b.1 * l)) - 1000;
        let i = k * u;
        let j = l * u;

        let small_input = Input{
            a: input.a,
            b: input.b,
            t: sub(input.t, add(times(input.a, i), times(input.b, j)))
        };

        if let Some(small_cost) = solve_small(&small_input) {
            let cost = small_cost + i * 3 + j;
            best = Some(min(best.unwrap_or(cost), cost));
        }
    }
    best
}

fn main()
{
    println!("{}", parse().iter().filter_map(solve).sum::<i64>());
}