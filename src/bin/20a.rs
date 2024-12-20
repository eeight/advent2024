use std::{collections::HashMap, fs::read_to_string};
use pathfinding::prelude::dijkstra_reach;

type V = (i32, i32);

static CHEAT_LEN: i32 = 20;

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

fn reach(f: &Vec<Vec<char>>, s: V) -> HashMap<V, usize>
{
    let next = |(i, j): &V, _: usize| -> Vec<(V, usize)> {
        let mut res = Vec::new();
        for a in &[-1, 1] {
            for b in &[-1, 1] {
                let i = i + (a + b)/ 2;
                let j = j + (a - b) / 2;

                if i >= 0 && (i as usize) < f.len() && j >= 0 && (j as usize) < f[0].len() && f[i as usize][j as usize] == '.' {
                    res.push(((i, j), 1));
                }
            }
        }
        res
    };

    dijkstra_reach(&s, next).map(|i| (i.node, i.total_cost)).collect()
}

fn solve(f: Vec<Vec<char>>, s: V, e: V) -> usize
{
    let s_reach = reach(&f, s);
    let e_reach = reach(&f, e);

    let def_path = *s_reach.get(&e).unwrap();
    
    println!("def: {}", def_path);

    let cheat = |x: V, y: V| -> usize {
        let cheat_len = (x.0 - y.0).abs() + (x.1 - y.1).abs();

        if x == y || cheat_len > CHEAT_LEN {
            return 0;
        }

        let x_part = match s_reach.get(&x) {
            Some(c) => c,
            None => return 0
        };
        let y_part = match e_reach.get(&y) {
            Some(c) => c,
            None => return 0
        };
        let path = x_part + (cheat_len as usize) + y_part;

        (path + 100 <= def_path) as usize
    };
    
    let mut count= 0;
    for i in 0..f.len() as i32 {
        for j in 0..f[0].len() as i32 {
            for k in 0..f.len() as i32 {
                for l in 0..f[0].len() as i32 {
                    count += cheat((i, j), (k, l));
                }
            }
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