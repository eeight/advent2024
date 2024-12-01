use std::fs::File;
use std::io::{BufRead, BufReader};

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
    
    let mut xs: Vec<i32> = pairs.iter().map(|(a, _)| *a).collect();
    let mut ys: Vec<i32> = pairs.iter().map(|(_, b)| *b).collect();

    xs.sort();
    ys.sort();

    let result: i32 = xs.iter().zip(ys.iter()).map(|(x, y)| (x - y).abs()).sum();

    println!("{}", result);
}