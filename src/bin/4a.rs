use std::fs::read_to_string;

fn matches(input: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let ms = |x: char| x == 'M' || x == 'S';

    if input[i + 1][j + 1] != 'A' ||
        !ms(input[i][j]) ||
        !ms(input[i][j + 2]) ||
        !ms(input[i + 2][j]) ||
        !ms(input[i + 2][j + 2])  {
        return false
    }

    (input[i][j] != 'M') != (input[i + 2][j + 2] != 'M') && (input[i][j + 2] != 'M') != (input[i + 2][j] != 'M')
}

fn main()
{
    let input : Vec<Vec<char>> = read_to_string("4.txt").unwrap().lines().map(|s| s.chars().collect()).collect();
    let mut result = 0;

    for i in 0..input.len() - 2 {
        for j in 0..input[0].len() - 2 {
            if matches(&input, i, j) {
                result += 1;
            }
        }
    }

    println!("{}", result)
}