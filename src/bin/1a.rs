use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main()
{
    let file = File::open("1.txt").unwrap();
    let reader = BufReader::new(file);

    let pairs: Vec<(i32, i32)> = reader.lines().filter_map(|maybe_line|
        {
            let line = maybe_line.ok()?;
            let mut ns = line.split_whitespace().map(|n| n.parse::<i32>().unwrap());
            let x = ns.next().unwrap();
            let y = ns.next().unwrap();
            Some((x, y))
        }).collect();
    
    let xs: Vec<i32> = pairs.iter().map(|(a, _)| *a).collect();
    let ys: Vec<i32> = pairs.iter().map(|(_, b)| *b).collect();

    let mut freqs: HashMap<i32, i32> = HashMap::new();

    for y in ys {
        freqs.entry(y).and_modify(|v| *v += 1).or_insert(1);
    }

    let result: i32 = xs.iter().map(|x| *x * freqs.get(x).unwrap_or(&0)).sum();

    println!("{}", result);
}