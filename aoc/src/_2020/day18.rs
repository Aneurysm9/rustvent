pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.input
            .trim()
            .lines()
            .map(|l| eval(parse(l, true)))
            .sum::<usize>()
            .to_string()
    }

    fn run_b(&self) -> String {
        self.input
            .trim()
            .lines()
            .map(|l| eval(parse(l, false)))
            .sum::<usize>()
            .to_string()
    }
}

fn eval(expr: Vec<char>) -> usize {
    let mut tmp = Vec::new();
    let mut ptr = 0;
    while ptr < expr.len() {
        match expr[ptr] {
            n if n.is_numeric() => tmp.push(n.to_string().parse::<usize>().unwrap()),
            '+' if tmp.len() >= 2 => {
                let a = tmp.pop().unwrap();
                let b = tmp.pop().unwrap();
                tmp.push(a + b);
            }
            '*' if tmp.len() >= 2 => {
                let a = tmp.pop().unwrap();
                let b = tmp.pop().unwrap();
                tmp.push(a * b);
            }
            o => panic!("Unexpected value enctountered: {}", o),
        }
        ptr += 1;
    }
    if tmp.len() > 1 {
        panic!("Unexpected result length {}: {:?}", tmp.len(), tmp);
    }
    tmp[0]
}

fn parse(expr: &str, part_a: bool) -> Vec<char> {
    let mut out = Vec::new();
    let mut ops = Vec::new();
    for c in expr.chars().filter(|c| *c != ' ') {
        match c {
            n if n.is_numeric() => out.push(n),
            '+' | '*' => {
                while !ops.is_empty() {
                    match ops.last() {
                        Some('+') | Some('*') if part_a => out.push(ops.pop().unwrap()),
                        Some('+') if c == '*' => out.push(ops.pop().unwrap()),
                        _ => break,
                    }
                }
                ops.push(c);
            }
            '(' => ops.push(c),
            ')' => {
                while *ops.last().unwrap() != '(' {
                    out.push(ops.pop().unwrap());
                }
                ops.pop();
            }
            o => panic!("Unexpected value enctountered: {}", o),
        }
    }
    while !ops.is_empty() {
        out.push(ops.pop().unwrap());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "18"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "18_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("26406"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("694122"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("5019432542701"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("70518821989947"));
    }
}
