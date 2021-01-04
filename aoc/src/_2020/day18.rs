pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.parse_input()
            .map(|o| o.value())
            .sum::<usize>()
            .to_string()
    }

    fn run_b(&self) -> String {
        // let exprs: Vec<Vec<&str>> = self
        //     .input
        //     .trim()
        //     .lines()
        //     .map(|l| {
        //         l.chars()
        //             .filter(|c| *c != ' ')
        //             .map(|c| {})
        //             .collect::<Vec<&str>>()
        //     })
        //     .collect();
        String::from("Not implemented")
    }
}

impl Runner {
    fn parse_input(&self) -> impl Iterator<Item = Binop> + '_ {
        self.input
            // String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
            .trim()
            .lines()
            .filter_map(|l| {
                let tokens: Vec<_> = l.chars().filter(|c| *c != ' ').collect();
                Binop::new(&tokens)
            })
    }
}

trait Value {
    fn value(&self) -> usize;
}

impl Value for usize {
    fn value(&self) -> usize {
        *self
    }
}

#[derive(Debug)]
enum Operation {
    Add,
    Mul,
}

struct Binop {
    lhs: Box<dyn Value>,
    rhs: Box<dyn Value>,
    op: Operation,
}

impl Value for Binop {
    fn value(&self) -> usize {
        match self.op {
            Operation::Add => self.lhs.value() + self.rhs.value(),
            Operation::Mul => self.lhs.value() * self.rhs.value(),
        }
    }
}

impl Binop {
    fn new(input: &[char]) -> Option<Binop> {
        if input.len() == 1 {
            return Some(Binop {
                lhs: Box::new(input[0].to_string().parse::<usize>().unwrap()),
                op: Operation::Add,
                rhs: Box::new(0),
            });
        }
        if input.len() < 3 {
            return None;
        }

        let mut ptr = input.len() - 1;
        let rhs: Box<dyn Value> = match input[ptr] {
            ')' => {
                let start = match_parens(&input, ptr);
                let rhs = Box::new(Binop::new(&input[start + 1..ptr]).unwrap());
                ptr = start;
                rhs
            }
            _ => Box::new(input[ptr].to_string().parse::<usize>().unwrap()),
        };
        if ptr == 0 {
            return Some(Binop {
                lhs: Box::new(0),
                op: Operation::Add,
                rhs,
            });
        }
        ptr -= 1;
        Some(Binop {
            lhs: Box::new(Binop::new(&input[..ptr]).unwrap()),
            op: match input[ptr] {
                '+' => Operation::Add,
                '*' => Operation::Mul,
                o => panic!("Unexpected operation {}", o),
            },
            rhs,
        })
    }
}

fn match_parens(input: &[char], end: usize) -> usize {
    let mut depth = 0;
    let mut ptr = end;
    loop {
        match input[ptr] {
            ')' => depth += 1,
            '(' => depth -= 1,
            _ => (),
        }
        if depth == 0 {
            return ptr;
        }
        ptr -= 1;
    }
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

    // #[test]
    // fn simple_b() {
    //     assert_eq!(simple().run_b(), String::from("694122"));
    // }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("5019432542701"));
    }

    // #[test]
    // fn real_b() {
    //     assert_eq!(new().run_b(), String::from("70518821989947"));
    // }
}
