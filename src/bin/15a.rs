use std::fs::read_to_string;

type V = (i32, i32);

struct State
{
    f: Vec<Vec<char>>,
    l: V,
}

fn parse() -> (State, Vec<V>)
{
    let input_text = read_to_string("15.txt").unwrap();

    let mut f: Vec<Vec<_>> = input_text
        .lines()
        .take_while(|l| l.len() > 1)
        .map(|l| l
                .chars()
                .flat_map(|c| match c {
                    '#' => "##".chars(),
                    'O' => "[]".chars(),
                    '.' => "..".chars(),
                    '@' => "@.".chars(),
                    _ => unreachable!()
                })
                .collect())
        .collect();
    let ms: Vec<_> = input_text
        .lines()
        .skip_while(|l| l.len() > 1)
        .skip(1)
        .flat_map(|s| s.chars())
        .map(|c| match c {
            '<' => (0, -1),
            '>' => (0, 1),
            '^' => (-1, 0),
            'v' => (1, 0),
            _ => unreachable!()
        }).collect();

    for i in 0..f.len() {
        for j in 0..f[i].len() {
            if f[i][j] == '@' {
                f[i][j] = '.';
                return (State {f, l: (i as i32, j as i32)}, ms);
            }
        }
    }

    unreachable!();
}

fn add(x: V, y: V) -> V
{
    (x.0 + y.0, x.1 + y.1)
}

impl State {
    fn apply(&mut self, m: V) {
        let next_l = add(self.l, m);
        let save_f = self.f.clone();
        if self.try_move(next_l, m) {
            self.l = next_l;
        } else {
            self.f = save_f;
        }
    }

    fn try_move(&mut self, l: V, m: V) -> bool {
        match self.get(l) {
            '.' => return true,
            '#' => return false,
            _ => ()
        };
    
        let xs = {
            let mut xs = vec![l];
            match self.get(l) {
                '[' => xs.push(add(l, (0, 1))),
                ']' => xs.push(add(l, (0, -1))),
                _ => unreachable!()
            }
            xs
        };

        for x in &xs {
            let next_x = add(*x, m);
            if xs.contains(&next_x) {
                continue;
            }
            if !self.try_move(next_x, m){
                return false;
            }
        }

        let cs: Vec<_> = xs.iter().map(|x| self.get(*x)).collect();
        for x in xs.iter() {
            self.set(*x, '.');
        }

        for (x, c) in xs.iter().zip(cs.iter()) {
            self.set(add(*x, m), *c);
        }
        true
    }

    fn get(&self, l: V) -> char {
        self.f[l.0 as usize][l.1 as usize]
    }

    fn set(&mut self, l: V, c: char) {
        self.f[l.0 as usize][l.1 as usize] = c;
    }

    fn dump(&self) {
        println!("====");
        for (i, l) in self.f.iter().enumerate() {
            if i as i32 != self.l.0 {
                println!("{}", String::from_iter(l.iter()))
            } else {
                println!(
                    "{}@{}",
                    String::from_iter(l.iter().take(self.l.1 as usize)),
                    String::from_iter(l.iter().skip(self.l.1 as usize + 1)))
            }
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..self.f.len() {
            for j in 0..self.f[i].len() {
                if self.f[i][j] == '[' {
                    score += i * 100 + j;
                }
            }
        }

        score
    }
}

fn solve(mut state: State, moves: &[V]) -> usize {
    for m in moves {
        //state.dump();
        state.apply(*m);
    }

    state.dump();
    state.score()
}

fn main()
{
    let (state, ms) = parse();
    println!("{}", solve(state, &ms));
}