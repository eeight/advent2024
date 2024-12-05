use std::fs::read_to_string;
use std::collections::HashSet;

fn parse() -> (HashSet<(i32, i32)>, Vec<Vec<i32>>)
{
    let input_text = read_to_string("5.txt").unwrap();
    let lines: Vec<_> = input_text.lines().collect();
    let before = HashSet::from_iter(
     lines.iter()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            let mut ps = line.split('|').map(|x| x.parse::<i32>().unwrap());
            let x = ps.next().unwrap();
            let y = ps.next().unwrap();
            (x, y)})
    );

    let prints: Vec<Vec<i32>> = lines.iter()
        .skip_while(|line| line.len() > 0)
        .skip(1)
        .map(|l| l.split(',').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    (before, prints)
}

fn is_good(before: &HashSet<(i32, i32)>, seq: &[i32]) -> bool {
    for i in 1..seq.len() {
        for j in 0..i {
            if before.contains(&(seq[i], seq[j])) {
                return false
            }
        }
    }
    return true
}

fn sort_pages(before: &HashSet<(i32, i32)>, input_seq: &[i32]) -> Vec<i32>
{
    let mut input: Vec<i32> = input_seq.to_vec();
    let mut result: Vec<i32> = Vec::new();

    while input.len() > 0
    {
        let mut advanced = false;

        for i in 0..input.len()
        {
            let mut can_first = true;
            for j in 0..input.len() {
                if before.contains(&(input[j], input[i])) {
                    can_first = false;
                    break
                }
            }

            if can_first {
                result.push(input[i]);
                input.remove(i);
                advanced = true;
                break;
            }
        }

        assert!(advanced);
    }

    result
}

fn main()
{
    let (before, prints) = parse();

    let mut sum = 0;

    for p in prints {
        if !is_good(&before, &p) {
            sum += sort_pages(&before, &p)[p.len() / 2]
        }
    }

    println!("{}", sum)
}