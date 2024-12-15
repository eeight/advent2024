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

    let mut f: Vec<Vec<_>> = input_text.lines().take_while(|l| l.len() > 1).map(|l| l.chars().collect()).collect();
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
        let mut l = add(self.l, m);
        loop {
            match self.get(l) {
                '#' => return,
                '.' => break,
                'O' => (),
                _ => unreachable!()
            }
            l = add(l, m);
        }

        l = add(self.l, m);
        self.l = l;

        if self.get(l) == '.' {
            // moving to a free space
            return;
        }
        assert!(self.get(l) == 'O');
        self.set(l, '.');
        loop {
            l = add(l, m);
            if self.get(l) == '.' {
                self.set(l, 'O');
                break;
            }
        }
    }

    fn get(&self, l: V) -> char {
        self.f[l.0 as usize][l.1 as usize]
    }

    fn set(&mut self, l: V, c: char) {
        self.f[l.0 as usize][l.1 as usize] = c;
    }

    fn dump(&self) {
        println!("====");
        for l in &self.f {
            println!("{}", String::from_iter(l.iter()))
        }
    }

    fn score(&self) -> usize {
        let mut score = 0;
        for i in 0..self.f.len() {
            for j in 0..self.f[i].len() {
                if self.f[i][j] == 'O' {
                    score += i * 100 + j;
                }
            }
        }

        score
    }
}

fn solve(mut state: State, moves: &[V]) -> usize {
    for m in moves {
        // state.dump();
        state.apply(*m);
    }

    // state.dump();
    state.score()
}

fn main()
{
    let (state, ms) = parse();
    println!("{}", solve(state, &ms));
}