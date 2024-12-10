use std::{collections::{HashMap, HashSet}, fs::read_to_string};

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

fn go(i: i32, j: i32, n: u32, fs: &HashMap<V, u32>, dest: &mut HashSet<V>) {
    if fs.get(&(i, j)) != Some(&n) {
        return;
    }
    if n == 9 {
        dest.insert((i, j));
        return;
    }

    for a in &[-1, 1] {
        for b in &[-1, 1] {
           go(i + (a + b) / 2, j + (a - b) / 2, n + 1, fs, dest);
        }
    }
}

fn count_trails(fs: &HashMap<V, u32>) -> usize {
    fs.keys().map(|(i, j)| {
        let mut dest = HashSet::new();
        go(*i, *j,0, fs, &mut dest);
        dest.len()
    }).sum()
}

fn main()
{
    let fs = parse();
    println!("{}", count_trails(&fs));
}