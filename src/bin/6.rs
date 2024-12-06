use std::fs::read_to_string;

type V = (i32, i32);
type F = Vec<Vec<char>>;

fn parse() -> (F, V, V)
{
    let input_text = read_to_string("6.txt").unwrap();
    let mut f: F = input_text.lines().map(|x| x.chars().collect()).collect();

    for i in 0..f.len() {
        for j in 0..f[0].len() {
            let dir = match f[i][j] {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                _ => continue
            };
            f[i][j] = '.';
            return (f, (i as i32, j as i32), dir)
        }
    }
    panic!("unreachable")
}

fn turn((i, j): V) -> V {
    (j, -i)
}

fn add((i, j): V, (k, m): V) -> V {
    (i + k, j + m)
}

fn walk(f: &F, mut loc: V, mut dir: V) -> usize
{
    let mut b: Vec<Vec<bool>> = vec![vec![false; f[0].len()]; f.len()];
    let mut count: usize = 0;

    let in_bounds= |(i, j): V| i >= 0 && (i as usize) < f.len() && j >= 0 && (j as usize) < f[0].len();
    let get = |(i, j): V| f[i as usize][j as usize];
    let mut mark = |(i, j): V| { let x = &mut b[i as usize][j as usize]; count += !*x as usize; *x = true;};

    loop {
        mark(loc);

        loc = loop {
            let next_loc = add(loc, dir);
            if !in_bounds(next_loc) {
                return count
            }
            if get(next_loc) == '.' {
                break next_loc;
            }
            dir = turn(dir)
        };
    }
}

fn main()
{
    let (f, loc, dir) = parse();
    println!("{}", walk(&f, loc, dir));
}