use std::{cmp::max, collections::{HashMap, HashSet}, fs::read_to_string};

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

fn solve(es: &[(String, String)]) -> String
{
    let mut ns: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut name_to_id = HashMap::new();
    let mut id_to_name = Vec::new();

    let mut get_id = |name| -> u32 {
        *name_to_id.entry(name)
            .or_insert_with(|| {
                let id = id_to_name.len() as u32;
                id_to_name.push(name);
                id
            })
    };

    for (a, b) in es {
        let a_id = get_id(a);
        let b_id = get_id(b);
        ns.entry(a_id).or_insert(HashSet::new()).insert(b_id);
        ns.entry(b_id).or_insert(HashSet::new()).insert(a_id);
    }

    for (a, a_out) in &ns {
        assert!(!a_out.contains(a));
    }

    let mut qs: Vec<HashSet<u32>> = ns.keys().map(|x| HashSet::from([*x])).collect();

    loop {
        println!("{}", qs.len());
        let mut next_qs = Vec::new();

        for q in &qs {
            let mut next_vs = ns.get(q.iter().next().unwrap()).unwrap().clone();
            let mut threshold = 0;
            for v in q {
                threshold = max(threshold, *v);
                next_vs = next_vs
                    .intersection(ns.get(v).unwrap())
                    .copied()
                    .filter(|v| *v > threshold)
                    .collect();
            }

            for next_v in next_vs {
                let mut next_q = q.clone();
                next_q.insert(next_v);
                next_qs.push(next_q);
            }
        }

        if next_qs.is_empty() {
            break;
        }

        qs = next_qs;
    }

    assert!(qs.len() == 1);
    let mut vs: Vec<&str> = qs[0].iter().map(|id| id_to_name[*id as usize].as_str()).collect();
    vs.sort();
    vs.join(",")
}

fn main()
{
    let es = parse();
    println!("{}", solve(&es));
}