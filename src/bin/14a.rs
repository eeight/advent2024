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

fn dump(vs: &[V]) {
    let mut field = vec![vec![' '; X_BOUND as usize]; Y_BOUND as usize];

    for (x, y) in vs {
        field[*y as usize][*x as usize] = '#';
    }

    for l in field {
        println!("{}", String::from_iter(l.iter()));
    }
}

fn sus(vs: &[V]) -> bool {
    let mut f = vec![vec![false; Y_BOUND as usize]; X_BOUND as usize];
    for (x,y) in vs {
        f[*x as usize][*y as usize] = true;
    }

    for c in f {
        let mut rn = 0;
        for n in c {
            if n {
                rn += 1;
                if rn > 10 {
                    return true;
                }
            } else {
                rn = 0;
            }
        }
    }

    false
}

fn solve(rs: &[Robot])
{
    for i in 0..1000000 {
        let vs: Vec<_> = rs.iter().map(|r| predict(r, i)).collect();
        if sus(&vs) {
            println!("{}", i);
            dump(&vs);
            break;
        }
    }
}

fn main()
{
    solve(&parse());
}