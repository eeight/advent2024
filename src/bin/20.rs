use std::fs::read_to_string;
use pathfinding::prelude::bfs;

type V = (i32, i32);

fn parse() -> (Vec<Vec<char>>, V, V)
{
    let input_text = read_to_string("20.txt").unwrap();

    let mut f: Vec<Vec<_>> = input_text.lines().map(|l| l.chars().collect()).collect();

    let mut s = (0, 0);
    let mut e = (0, 0);
    for i in 0..f.len() {
        for j in 0..f[i].len() {
            match f[i][j] {
                'S' => s = (i as i32, j as i32),
                'E' => e = (i as i32, j as i32),
                _ => continue
            };
            f[i][j] = '.'; 
        }
    }

    (f, s, e)
}

fn sp(f: &Vec<Vec<char>>, s: V, e: V, cheat_x: V, cheat_y: V) -> usize
{
    let next = |(i, j): &V | -> Vec<V> {
        if (*i, *j) == cheat_x {
            return vec![cheat_y];
        }
        let mut res = Vec::new();
        for a in &[-1, 1] {
            for b in &[-1, 1] {
                let i = i + (a + b)/ 2;
                let j = j + (a - b) / 2;

                if i >= 0 && (i as usize) < f.len() && j >= 0 && (j as usize) < f[0].len() && f[i as usize][j as usize] == '.' {
                    res.push((i, j));
                }
            }
        }
        res
    };

    let success = |n: &V| *n == e;

    if let Some(v) = bfs(&s, next, success) {
        v.len()
    } else {
        1000000
    }
}

fn solve(f: Vec<Vec<char>>, s: V, e: V) -> usize
{
    let def_path = sp(&f, s, e, (-1, -1), (-1, -1));
    println!("def: {}", def_path);

    let cheat = |x, y| {
        let path = sp(&f, s, e, x, y);
        (path + 100 <= def_path) as usize
    };
    
    let mut count= 0;
    for i in 0..f.len() as i32 {
        for j in 0..f[0].len() as i32 {
            let x = (i, j);
            count +=
                cheat(x, (i + 1, j)) + cheat(x, (i, j + 1))+
                cheat((i + 1, j), x) + cheat((i, j + 1), x);
        }
    }

    count
}

fn main()
{
    let (f, s, e) = parse();
    let result = solve(f, s, e);
    println!("{}", result);
}