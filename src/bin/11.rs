use std::fs::read_to_string;

fn parse() -> Vec<u64>
{
    let input_text = read_to_string("11.txt").unwrap();

    input_text.split_whitespace().map(|w| w.parse::<u64>().unwrap()).collect()
}

fn blink(xs: &[u64]) -> Vec<u64> {
    let mut ys = Vec::new();

    for x in xs {
        if *x == 0 {
            ys.push(1);
            continue;
        }
        let ds = x.to_string();
        if ds.len() % 2 == 0 {
            ys.push(ds[..ds.len() / 2].parse::<u64>().unwrap());
            ys.push(ds[ds.len() / 2..].parse::<u64>().unwrap());
        } else {
            ys.push(x * 2024);
        }
    }

    ys
}

fn main()
{
    let mut fs = parse();
    for _ in 0..25 {
        fs = blink(&fs); 
    }
    println!("{}", fs.len());
}