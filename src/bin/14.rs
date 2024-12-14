use std::fs::read_to_string;

type V = (i32, i32);

#[derive(Debug)]
struct Robot {
    p: V,
    v: V
}

const X_BOUND: i32 = 101;
const Y_BOUND: i32 = 103;

fn parse() -> Vec<Robot>
{
    let input_text = read_to_string("14.txt").unwrap();

    let parse_v = |l: &str| -> V {
        let l = &l[2..];
        let (x, y) = l.split_once(',').unwrap();
        (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
    };

    input_text
        .lines()
        .filter(|l| l.len() > 1)
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            Robot{ p: parse_v(p), v: parse_v(v)}
        })
        .collect()
}

fn predict(r: &Robot, n: i32) -> V
{
    let predict_coord = |x: i32, v: i32, m: i32| -> i32 {
        (x + v * n).rem_euclid(m) 
    };

    (predict_coord(r.p.0, r.v.0, X_BOUND), predict_coord(r.p.1, r.v.1, Y_BOUND))
}

fn solve(rs: &[Robot]) -> i32
{
    let mut qs = vec![0; 4];
    for r in rs {
        let v = predict(r, 100);
        if v.0 == X_BOUND / 2 || v.1 == Y_BOUND / 2 {
            continue;
        }
        let x = (v.0 < X_BOUND / 2) as usize;
        let y = (v.1 < Y_BOUND / 2) as usize;
        qs[x + y*2] += 1;
    }

    qs.iter().product()
}

fn main()
{
    println!("{}", solve(&parse()));
}