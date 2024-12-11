use std::{collections::HashMap, fs::read_to_string};

fn parse() -> Vec<u64>
{
    let input_text = read_to_string("11_.txt").unwrap();

    input_text.split_whitespace().map(|w| w.parse::<u64>().unwrap()).collect()
}

struct Solver {
    memo: HashMap<(u64, usize), usize>
}

impl Solver {
    fn new() -> Self {
        Solver { memo: HashMap::new() }
    }

    pub fn get_expansion(&mut self, x: u64, n: usize) -> usize {
        if let Some(a) = self.memo.get(&(x, n)) {
            return *a;
        }
        let result = self.get_expansion_impl(x, n);
        self.memo.insert((x, n), result);
        result
    }

    fn get_expansion_impl(&mut self, x: u64, n: usize) -> usize {
        if n == 0 {
            return 1;
        }
        let mut cont = |m| self.get_expansion(m, n - 1);
        if x == 0 {
            return cont(1);
        }
        let ds = x.to_string();
        if ds.len() % 2 == 0 {
            cont(ds[..ds.len() / 2].parse::<u64>().unwrap()) +
                cont(ds[ds.len() / 2..].parse::<u64>().unwrap())
        } else {
            cont(x * 2024)
        }
    }
}

fn solve(xs: &[u64], n: usize) -> usize {
    let mut solver = Solver::new();

    xs.iter().map(|x| solver.get_expansion(*x, n)).sum()
}

fn main()
{
    let fs = parse();
    println!("{}", solve(&fs, 75));
}