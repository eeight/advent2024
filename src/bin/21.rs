use std::{collections::HashMap, fs::read_to_string};

fn parse() -> Vec<String>
{
    let input_text = read_to_string("21.txt").unwrap();
    input_text.lines().map(str::to_string).collect()
}

struct Encoder<'a> {
    coord: &'a HashMap<char, (i32, i32)>,
    hole: (i32, i32),
    result: Vec<char>,
    last_i: bool,
    i: i32,
    j: i32
}

impl <'a> Encoder<'a> {
    fn new(coord: &'a HashMap<char, (i32, i32)>, hole: (i32, i32), (i, j): (i32, i32)) -> Self {
        Encoder{ coord, hole, result: Vec::new(), last_i: false, i, j }
    }

    fn next(&mut self, c: char) {
        let (next_i, next_j) = *self.coord.get(&c).unwrap();

        if self.last_i || (self.i, next_j) == self.hole {
            self.trim_i(next_i);
            self.trim_j(next_j);
        } else {
            self.trim_j(next_j);
            self.trim_i(next_i);
        }
        self.result.push('A');
    }

    fn trim_i(&mut self, next_i: i32) {
        if self.i != next_i {
            self.last_i = true;
        }
        while self.i > next_i {
            self.result.push('^');
            self.i -= 1;
            assert!((self.i, self.j) != self.hole);
        }
        while self.i < next_i {
            self.result.push('v');
            self.i += 1;
            assert!((self.i, self.j) != self.hole);
        }
    }

    fn trim_j(&mut self, next_j: i32) {
        if self.j != next_j {
            self.last_i = false;
        }

        while self.j > next_j {
            self.result.push('<');
            self.j -= 1;
            assert!((self.i, self.j) != self.hole);
        }
        while self.j < next_j {
            self.result.push('>');
            self.j += 1;
            assert!((self.i, self.j) != self.hole);
        }
    }

    fn encode(&mut self, code: &str) -> String
    {
        for c in code.chars() {
            self.next(c)
        }
        String::from_iter(self.result.iter())
    }

    fn encode_cont<C: FnMut(String)>(&mut self, code: &[char], cont: &mut C)
    {
        if code.len() == 0 {
            cont(String::from_iter(self.result.iter()));
            return;
        }

        let (next_i, next_j) = *self.coord.get(&code[0]).unwrap();

        if (next_i, self.j) != self.hole {
            self.encode_tail_save_restore(true, next_i, next_j, code, cont);
        }
        if (self.i, next_j) != self.hole {
            self.encode_tail_save_restore(false, next_i, next_j, code, cont);
        }
    }

    fn encode_tail_save_restore<C: FnMut(String)>(&mut self, i_first: bool, next_i: i32, next_j: i32, code: &[char], cont: &mut C) {
        let save_len =  self.result.len();
        let save_i = self.i;
        let save_j = self.j;

        if i_first {
            self.trim_i(next_i);
        }
        self.trim_j(next_j);
        if !i_first {
            self.trim_i(next_i);
        }

        self.result.push('A');
        self.encode_cont(&code[1..], cont);
        self.i = save_i;
        self.j = save_j;
        self.result.truncate(save_len);

    }

    fn encode_all(&mut self, code: &str) -> Vec<String> {
        let mut result = Vec::new();
        self.encode_cont(&code.chars().collect::<Vec<char>>(), &mut |s| { result.push(s); });
        result
    }
}

fn encode_num(code: &str) -> Vec<String> {
    let coord = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2))
        ]);
    Encoder::new(&coord, (3, 0), (3, 2)).encode_all(&code)
}

fn encode_dir(code: &str) -> String {
    let coord = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
        ]);
    Encoder::new(&coord, (0, 0), (0, 2)).encode(code)
}

fn solve(codes: &[String]) -> u64
{
    let mut result = 0;

    for code in codes {
        let mut shortest = String::new();
        for num in encode_num(code) {
            let dir1 = encode_dir(&num);
            let dir2 = encode_dir(&dir1);
            if shortest.len() == 0 || shortest.len() > dir2.len() {
                shortest = dir2;
            }
        }
        let x: u64 = code.chars().filter(|c| c.is_digit(10)).collect::<String>().parse::<u64>().unwrap();
        println!("{} * {} = {}", x, shortest.len(), x * shortest.len() as u64);
        result += x * shortest.len() as u64;
    }

    result
}

fn main()
{
    let codes = parse();
    println!("{}", solve(&codes));
}