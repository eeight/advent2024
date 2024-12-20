use std::fs::read_to_string;

fn parse() -> (Vec<String>, Vec<String>)
{
    let input_text = read_to_string("19.txt").unwrap();
    let mut lines = input_text.lines();

    let first = lines.next().unwrap();
    let towels = first.split(", ").map(|l| l.to_string()).collect();

    (towels, lines.filter(|l| l.len() > 0).map(|l| l.to_string()).collect())
}

fn possible(towels: &[String], p: &str) -> bool {
    let mut pref = vec![false; p.len() + 1];

    pref[0] = true;

    for i in  1..p.len() + 1 {
        for t in towels {
            if p[..i].ends_with(t) && pref[i - t.len()] {
                pref[i] = true;
                break;
            }
        }
    }

    pref[p.len()]
}

fn main()
{
    let (towels, patterns) = parse();
    let result = patterns.iter().filter(|p| possible(&towels, p)).count();
    println!("{}", result);
}