use std::fs::read_to_string;

fn parse() -> (Vec<String>, Vec<String>)
{
    let input_text = read_to_string("19.txt").unwrap();
    let mut lines = input_text.lines();

    let first = lines.next().unwrap();
    let towels = first.split(", ").map(|l| l.to_string()).collect();

    (towels, lines.filter(|l| l.len() > 0).map(|l| l.to_string()).collect())
}

fn possible(towels: &[String], p: &str) -> u64 {
    let mut pref = vec![0; p.len() + 1];

    pref[0] = 1;

    for i in  1..p.len() + 1 {
        for t in towels {
            if p[..i].ends_with(t) {
                pref[i] += pref[i - t.len()];
            }
        }
    }

    pref[p.len()]
}

fn main()
{
    let (towels, patterns) = parse();
    let result: u64 = patterns.iter().map(|p| possible(&towels, p)).sum();
    println!("{}", result);
}