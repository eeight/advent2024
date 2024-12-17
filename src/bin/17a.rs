use std::fs::read_to_string;

/*
impl State
{
    fn step(&mut self) -> (u64, bool) {
        let x = self.a % 8;
        self.b = x ^ (self.a / (1 << (x ^ 1))) ^ 5;
        self.a /= 8;
        
        (self.b % 8, self.a > 0)
    }
}
*/

fn encode(min_x: u64, a: u64, ns: &[u64]) -> Option<u64> {
    if ns.len() == 0 {
        return Some(a);
    }
    let n = ns[0];

    for x in min_x..8 {
        let a = a * 8 + x;
        if (x ^ (a / (1 << (x ^ 1))) ^ 5) % 8 == n {
            let a = encode(0, a, &ns[1..]);
            if a.is_some() {
                return a;
            }
        }
    }

    None
}

fn parse() -> Vec<u64>
{
    let input_text = read_to_string("17.txt").unwrap();
    let lines: Vec<_> = input_text
        .lines()
        .filter(|l|l.len() > 1)
        .map(|l| l.split_once(": ").unwrap().1)
        .collect();
    lines[3].split(",").map(|x| x.parse::<u64>().unwrap()).collect()
}

fn main()
{
    println!("{:?}", encode(1, 0, &parse().iter().copied().rev().collect::<Vec<_>>()));
}