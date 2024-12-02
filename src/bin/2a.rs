use std::fs::read_to_string;

fn is_safe(vec: &[i32]) -> bool
{
    let is_small_diff = vec.windows(2).all(|pair| { let adiff = (pair[0] - pair[1]).abs(); adiff >= 1 && adiff <= 3} );
    let is_inc = vec.iter().is_sorted();
    let is_dec = vec.iter().rev().is_sorted();

    is_small_diff && (is_inc || is_dec)
}

fn try_remove_n(vec: &[i32]) -> Vec<Vec<i32>>
{
    let mut x: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();

    for i in 0..vec.len()
    {
        x.push([&vec[0..i], &vec[i + 1..]].concat());
    }

    x
}

fn is_maybe_safe(vec: &[i32]) -> bool
{
    try_remove_n(vec).iter().any(|x| is_safe(x))
}


fn main()
{
    let vecs: Vec<Vec<i32>> = read_to_string("2.txt").unwrap().lines().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()).collect();
    let result = vecs.iter().filter(|x| is_maybe_safe(*x)).count();
    println!("{}", result);
}