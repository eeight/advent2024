use std::fs::read_to_string;
use itertools::Itertools;

fn parse() -> Vec<Option<usize>>
{
    let input_text = read_to_string("9.txt").unwrap() + "0";
    let mut result = Vec::new();

    for (file_id, (file_len, space_len)) in input_text.chars().tuples().enumerate() {
        for _ in 0..file_len.to_digit(10).unwrap() {
            result.push(Some(file_id));
        }
        for _ in 0..space_len.to_digit(10).unwrap_or(0) {
            result.push(None);
        }
    }

    result
}

fn solve(mut fs: Vec<Option<usize>>) -> usize
{
    let mut i: usize = 0;
    let mut j: usize = fs.len() - 1;

    while i < j {
        if fs[i].is_some() {
            i += 1;
            continue
        }
        if fs[j].is_none() {
            j -= 1;
            continue;
        }
        fs.swap(i, j);
    }

    fs.iter().enumerate().map(|(i, k)| i * k.unwrap_or(0)).sum()
}

fn main()
{
    let fs = parse();
    println!("{}", solve(fs));
}