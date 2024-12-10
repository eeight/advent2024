use std::{collections::HashMap, fs::read_to_string};

type V = (i32, i32);

fn parse() -> HashMap<V, u32>
{
    let input_text = read_to_string("10.txt").unwrap();

    HashMap::from_iter(input_text.lines().enumerate().flat_map(|(i, l)| {
        l.chars()
            .enumerate()
            .filter_map(move |(j, c) | Some(((i as i32, j as i32), c.to_digit(10)?)))
       }))
}

fn go(i: i32, j: i32, n: u32, fs: &HashMap<V, u32>) -> usize {
    if fs.get(&(i, j)) != Some(&n) {
        return 0;
    }
    if n == 9 {
        return 1;
    }
    let mut count = 0;

    for di in &[-1, 0, 1] {
        for dj in &[-1, 0, 1] {
            if *di == 0 || *dj == 0 {
                count += go(i + di, j + dj, n + 1, fs);
            }
        }
    }
    count
}

fn count_trails(fs: &HashMap<V, u32>) -> usize {
    fs.keys().map(|(i, j)| go(*i, *j, 0, fs)).sum()
}

fn main()
{
    let fs = parse();
    println!("{}", count_trails(&fs));
}