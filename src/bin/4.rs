use std::fs::read_to_string;

fn matches(input: &Vec<Vec<char>>, input_pattern: &[char], input_i: usize, input_j: usize, di: i32, dj: i32) -> bool {
    let mut pattern = input_pattern;
    let mut i = input_i;
    let mut j = input_j;


    loop {
        if input[i][j] != pattern[0]
        {
            return false
        }

        pattern = &pattern[1..];

        if pattern.len() == 0 {
            return true
        }

        if (di < 0 && i == 0) || (dj < 0 && j == 0)
        {
            return false;
        }

        i = (i as i32 + di) as usize;
        j = (j as i32 + dj) as usize;

        if i >= input.len() || j >= input[i].len()
        {
            return false
        }
    }
}

fn main()
{
    let input : Vec<Vec<char>> = read_to_string("4.txt").unwrap().lines().map(|s| s.chars().collect()).collect();
    let pattern: Vec<char> = "XMAS".chars().collect();
    let mut result = 0;

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            for di in -1..2 {
                for dj in -1..2 {
                    if di == 0 && dj == 0 {
                        continue
                    }
                    if matches(&input, &pattern, i, j, di, dj) {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("{}", result)
}