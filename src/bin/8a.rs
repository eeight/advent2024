use std::{collections::HashMap, fs::read_to_string};

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

fn sub_v(x: V, y: V) -> V
{
    (x.0 - y.0, x.1 - y.1)
}

fn solve(antennas: HashMap<char, Vec<V>>, maxi: i32, maxj: i32) -> usize
{
    let mut count: usize = 0;

    let in_line = |a: V, x: V, y: V| {
        let d = sub_v(y, x);
        let da = sub_v(x, a);

        let k = da.0 / d.0;

        da.0 == d.0 * k && da.1 == d.1 * k
    };

    for i in 0..maxi {
        'square: for j in 0..maxj {
            for (_, locs) in antennas.iter() {
                for x in locs {
                    for y in locs {
                        if x != y && in_line((i, j), *x, *y) {
                            count += 1;
                            continue 'square;
                        }
                    }
                }
            }        
        }
    }

    count
}

fn main()
{
    let (a, maxi, maxj) = parse();
    println!("{}", solve(a, maxi, maxj));
}