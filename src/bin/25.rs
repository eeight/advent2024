use std::fs::read_to_string;

type Profile = Vec<u32>;

fn parse() -> (Vec<Profile>, Vec<Profile>)
{
    let input_text = read_to_string("25.txt").unwrap();
    
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let mut buffer = Vec::new();

    let mut finalize = |buffer: &mut Vec<Vec<char>>| {
        let profile = (0..buffer[0].len())
            .map(|i| {
                ((0..buffer.len()).filter(|j| buffer[*j][i] == '#').count() - 1) as u32
            }).collect();
        if buffer[0][0] == '.' {
            keys.push(profile);
        } else {
            locks.push(profile);
        }
        buffer.clear();
    };

    for line in input_text.lines() {
        if line.is_empty() {
            finalize(&mut buffer);
        } else {
            buffer.push(line.chars().collect());
        }
    }

    finalize(&mut buffer);

    (keys, locks)
}

fn solve(keys: &[Profile], locks: &[Profile]) -> usize
{
    let mut result = 0;

    for key in keys {
        for lock in locks {
            result += key.iter().zip(lock.iter()).all(|(x, y)| x + y <= 5) as usize;
        }
    }

    result
}

fn main()
{
    let (keys, locks) = parse();
    println!("{}", solve(&keys, &locks));
}