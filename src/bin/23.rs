use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn parse() -> Vec<(String, String)>
{
    let input_text = read_to_string("23.txt").unwrap();
    input_text
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect()
}

fn solve(es: &[(String, String)]) -> usize
{
    let mut ns: HashMap<&str, HashSet<&str>> = HashMap::new();
    for (a, b) in es {
        ns.entry(&a).or_insert(HashSet::new()).insert(&b);
        ns.entry(&b).or_insert(HashSet::new()).insert(&a);
    }

    let mut count = 0;

    for (a, a_out) in &ns {
        for b in a_out {
            for c in a_out.intersection(&ns.get(b).unwrap()) {
                if c == a || c == b {
                    continue;
                }
                if !a.starts_with('t') && !b.starts_with('t') && !c.starts_with('t') {
                    continue;
                }
                count += 1;
            }
        }
    }

    assert!(count % 6 == 0);
    count / 6
}

fn main()
{
    let es = parse();
    println!("{}", solve(&es));
}