use std::fs::read_to_string;
use itertools::Itertools;

fn parse() -> (Vec<(usize, usize)>, Vec<(usize, usize)>)
{
    let input_text = read_to_string("9.txt").unwrap();
    let mut fs = Vec::new();
    let mut hs = Vec::new();

    let mut loc: usize = 0;
    for (file_len, space_len) in input_text.chars().map(|x| x.to_digit(10).unwrap_or(0) as usize).tuples() {
        fs.push((loc, file_len));
        loc += file_len;
        hs.push((loc, space_len));
        loc += space_len;
    }

    (fs, hs)
}

fn solve(fs: &[(usize, usize)], mut hs: Vec<(usize, usize)>) -> usize
{
    fs.iter().enumerate().rev().map(|(file_id, (file_loc, file_len))| {
        for hole_id in 0..hs.len() {
            let (hole_loc, hole_len) = hs[hole_id];

            if hole_len >= *file_len && hole_loc < *file_loc {
                hs[hole_id].0 += file_len;
                hs[hole_id].1 -= file_len;
                return (file_id, (hole_loc, *file_len))
            }
        }
        (file_id, (*file_loc, *file_len))
    }).map(|(file_id, (loc, size))| -> usize {
        (loc..(loc + size)).map(|l| l * file_id).sum()
    }).sum()
}

fn main()
{
    let (fs, hs) = parse();
    println!("{}", solve(&fs, hs));
}