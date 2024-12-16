use std::fs::read_to_string;
use pathfinding::prelude::dijkstra;

type V = (usize, usize);

fn parse() -> (Vec<Vec<char>>, V, V)
{
    let input_text = read_to_string("16.txt").unwrap();

    let mut f: Vec<Vec<_>> = input_text.lines().map(|l| l.chars().collect()).collect();

    let mut s = (0, 0);
    let mut e = (0, 0);
    for i in 0..f.len() {
        for j in 0..f[i].len() {
            if f[i][j] == 'S' {
                s = (i, j);
                f[i][j] = '.';
            } else if f[i][j] == 'E' {
                e = (i, j);
                f[i][j] = '.';
            }
        }
    }

    (f, s, e)
}

fn dir_to_v(d: i32) -> (i32, i32) {
    match d {
        0 => (-1, 0),
        1 => (0, 1),
        2 => (1, 0),
        3 => (0, -1),
        _ => unreachable!()
    }
}

fn solve(f: Vec<Vec<char>>, s: V, e: V) -> i32 {
    let next = |((i, j), d): &(V, i32) | -> Vec<((V, i32), i32)> {
        let mut res = Vec::new();
        let (di, dj) = dir_to_v(*d);
        let i_ = (*i as i32 + di) as usize;
        let j_ = (*j as i32 + dj) as usize;

        if f[i_][j_] == '.' {
            res.push((((i_, j_), *d), 1));
        }

        res.push((((*i, *j), (*d + 1) % 4), 1000));
        res.push((((*i, *j), (*d + 3) % 4), 1000));

        res
    };

    let success = |(n, _): &(V, i32)| *n == e;

    let (_, c) = dijkstra(&(s, 1), next, success).unwrap();

    c
}

fn main()
{
    let (f, s, e) = parse();
    println!("{}", solve(f, s, e));
}