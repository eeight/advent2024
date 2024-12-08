use std::{collections::{HashMap, HashSet}, fs::read_to_string};

type V = (i32, i32);

fn parse() -> (HashMap<char, Vec<V>>, i32, i32)
{
    let input_text = read_to_string("8.txt").unwrap();
    let mut a: HashMap<char, Vec<V>> = HashMap::new();

    for (i, line) in input_text.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                a.entry(c)
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    (a, input_text.lines().count() as i32, input_text.lines().next().unwrap().len() as i32)
}

fn solve(antennas: HashMap<char, Vec<V>>, maxi: i32, maxj: i32) -> usize
{
    let mut ps: HashSet<V> = HashSet::new();

    let in_bounds = |(i, j): V| {
        i >= 0 && j >= 0 && i < maxi && j < maxj
    };

    for (_, locs) in antennas.iter() {
        for x in locs {
            for y in locs {
                if x == y {
                    continue
                }
                let di = y.0 - x.0;
                let dj = y.1 - x.1;
                for p in &[(x.0 - di, x.1 - dj), (y.0 + di, y.1 + dj)] {
                    if in_bounds(*p) {
                        ps.insert(*p);
                    }
                }
            }
        }
    }

    ps.len()
}

fn main()
{
    let (a, maxi, maxj) = parse();
    println!("{}", solve(a, maxi, maxj));
}