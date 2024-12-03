use std::fs::read_to_string;

struct Parser
{
    sum: i32,
    enabled: bool
}

impl Parser
{
    fn parse(&mut self, s: &str)
    {
        if s.is_empty()
        {
            return
        }
        match s.strip_prefix("do()")
        {
            Some(s) => { self.enabled = true; return self.parse(s)}
            _ => ()
        }
        match s.strip_prefix("don't()")
        {
            Some(s) => { self.enabled = false; return self.parse(s)}
            _ => ()
        }
        let Some(s) = s.strip_prefix("mul(") else { return self.parse(&s[1..]) };
        let comma_pos = match s.find(',')
        {
            Some(x) if x <= 3 => x,
            _ => return self.parse(s)
        };
        let Ok(x) = s[..comma_pos].parse::<i32>() else {
            return self.parse(&s[comma_pos + 1..])
        };
        let s = &s[comma_pos + 1..];
        let close_bracket_pos = match s.find(')')
        {
            Some(x) if x <= 3 => x,
            _ =>  return self.parse(s)
        };
        let Ok(y) = s[..close_bracket_pos].parse::<i32>() else {
            return self.parse(&s[close_bracket_pos + 1..])
        };
        if self.enabled {
            self.sum += x * y;
        }
        self.parse(&s[close_bracket_pos + 1..])
    }
}

fn main()
{
    let input = read_to_string("3.txt").unwrap();
    let mut p = Parser{sum: 0, enabled: true};
    p.parse(&input);
    println!("{}", p.sum);
}