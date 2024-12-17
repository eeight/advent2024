use std::fs::read_to_string;

struct State
{
    a: u64,
    b: u64,
    c: u64,
    ip: usize,
    mem: Vec<u64>
}

impl State
{
    fn step(&mut self) -> (Option<u64>, bool) {
        if self.ip + 1 >= self.mem.len() {
            return (None, false)
        }

        let mut next_ip = self.ip + 2;
        let op = self.mem[self.ip + 1];
        let combo = || self.combo(self.ip + 1);
        let mut out = None;

        match self.mem[self.ip] {
            0 => self.a = self.a / (1 << combo()),
            1 => self.b ^= op,
            2 => self.b = combo() % 8,
            3 => if self.a > 0 { next_ip = op as usize },
            4 => self.b = self.b ^ self.c,
            5 => out = Some(combo() % 8),
            6 => self.b = self.a / (1 << combo()),
            7 => self.c = self.a / (1 << combo()),
            _ => unreachable!()
        }

        self.ip = next_ip;
        (out, true)
    }

    fn combo(&self, ptr: usize) -> u64 {
        match self.mem[ptr as usize] {
            v if v < 4 => v,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            v => panic!("{}", v)
        }
    }
}

fn parse() -> State
{
    let input_text = read_to_string("17.txt").unwrap();
    let lines: Vec<_> = input_text
        .lines()
        .filter(|l|l.len() > 1)
        .map(|l| l.split_once(": ").unwrap().1)
        .collect();

    State {
        a: lines[0].parse::<u64>().unwrap(),
        b: lines[1].parse::<u64>().unwrap(),
        c: lines[2].parse::<u64>().unwrap(),
        ip: 0,
        mem: lines[3].split(",").map(|x| x.parse::<u64>().unwrap()).collect()
    }
}

fn solve(mut state: State) -> String
{
    let mut vs = Vec::new();
    loop {
        let (out, cont) = state.step();
        if !cont {
            break;
        }
        if let Some(x) = out {
            vs.push(x.to_string());
        } 
    }

    vs.join(",")
}

fn main()
{
    let state = parse();
    println!("{}", solve(state));
}