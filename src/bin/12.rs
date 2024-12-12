use std::fs::read_to_string;

fn parse() -> Vec<Vec<char>>
{
    let input_text = read_to_string("12.txt").unwrap();

    input_text.lines().map(|l| l.chars().collect()).collect()
}

struct Solver {
    f: Vec<Vec<char>>,
    u: Vec<Vec<bool>>,
    n: i32,
    m: i32
}

impl Solver {
    fn new(f: Vec<Vec<char>>) -> Self {
        let n = f.len();
        let m = f[0].len();
        Solver {
            f: f,
            u: vec![vec![false; m]; n],
            n: n as i32,
            m: m as i32
        }
    }

    fn get(&self, i: i32, j: i32) -> Option<char> {
        if i < 0  || i >= self.n || j < 0 || j >= self.m {
            None
        } else {
            Some(self.f[i as usize][j as usize])
        }
    }

    fn go(&mut self, i: i32, j: i32, c: char) -> (usize, usize) {
        if self.get(i, j) != Some(c) || self.u[i as usize][j as usize] {
            return (0, 0);
        }

        self.u[i as usize][j as usize] = true;

        let mut p = 0;
        let mut a = 1;

        for u in &[-1, 1] {
            for v in &[-1, 1] {
                let i = i + (u + v) / 2;
                let j = j + (u - v) / 2;
                if self.get(i, j) != Some(c) {
                    p += 1;
                }
                let (pn, an) = self.go(i, j, c);
                p += pn;
                a += an;
            }
        }
        (p, a)
    }

    fn solve(&mut self) -> usize {
        let mut result = 0;
        for i in 0..self.n {
            for j in 0..self.m {
                let (p, a) = self.go(i, j, self.f[i as usize][j as usize]);
                result += p * a
            }
        }
        result
    }
}

fn main()
{
    let mut solver = Solver::new(parse());
    println!("{}", solver.solve());
}