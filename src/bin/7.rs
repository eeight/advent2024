use std::fs::read_to_string;


fn parse() -> Vec<(i64, Vec<i64>)>
{
    let input_text = read_to_string("7.txt").unwrap();
    input_text.lines().map(|line| {
        let (target, args) = line.split_once(": ").unwrap();
        (target.parse().unwrap(), args.split(' ').map(|x| x.parse().unwrap()).collect())
    }).collect()
}

fn solve(acc: i64, t: i64, args: &[i64]) -> bool
{
    if args.len() == 0 {
        return t == acc;
    }

    solve(acc + args[0], t, &args[1..]) || solve(acc * args[0], t, &args[1..])
}

fn main()
{
    let input = parse();
    let count: i64 = input.iter().map(|(t, a)| {
        (solve(0, *t, a) as i64) * t
    }).sum();
    println!("{}", count)
}