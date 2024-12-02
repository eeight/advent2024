use std::fs::read_to_string;

fn is_safe(vec: &[i32]) -> bool
{
    let is_small_diff = vec.windows(2).all(|pair| { let adiff = (pair[0] - pair[1]).abs(); adiff >= 1 && adiff <= 3} );
    let is_inc = vec.iter().is_sorted();
    let is_dec = vec.iter().rev().is_sorted();

    is_small_diff && (is_inc || is_dec)
}

fn main()
{
    let vecs: Vec<Vec<i32>> = read_to_string("2.txt").unwrap().lines().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()).collect();
    let result = vecs.iter().filter(|x| is_safe(*x)).count();
    println!("{}", result);
}