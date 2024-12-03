use std::fs::read_to_string;

fn parse_muls(s: &str, sum: &mut i32)
{
    if s.is_empty()
    {
        return
    }
    if !s.starts_with("mul(")
    {
        return parse_muls(&s[1..], sum)
    }
    let s = &s[4..];
    let comma_pos = match s.find(',')
    {
        Some(x) if  x <= 3 => x,
        _ => return parse_muls(s, sum)
    };
    let x = match s[..comma_pos].parse::<i32>() {
        Ok(x) => x,
        _ => return parse_muls(&s[comma_pos + 1..], sum)
    };
    let s = &s[comma_pos + 1..];
    let close_bracket_pos = match s.find(')')
    {
        Some(x) if x <= 3 => x,
        _ =>  return parse_muls(s, sum)
    };
    let y = match s[..close_bracket_pos].parse::<i32>()
    {
        Ok(y) => y,
        _ => return parse_muls(&s[close_bracket_pos + 1..], sum)
    };
    *sum += x * y;
    parse_muls(&s[close_bracket_pos + 1..], sum)
}

fn main()
{
    let input = read_to_string("3.txt").unwrap();
    let mut sum: i32 = 0;
    parse_muls(&input, &mut sum);
    println!("{}", sum);
}