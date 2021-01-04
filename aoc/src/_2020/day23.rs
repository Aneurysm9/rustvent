pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let seed: Vec<usize> = self
            .input
            .trim()
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let mut cups = vec![0; 10];
        for (i, v) in seed.iter().enumerate() {
            cups[*v] = seed[(i + 1) % seed.len()];
        }
        let mut cur = seed[0];
        for _ in 0..100 {
            cur = round(&mut cups, cur);
        }
        cur = 1;
        let mut out = String::from("");
        for _ in 0..8 {
            out = format!("{}{}", out, cups[cur]);
            cur = cups[cur];
        }
        out
    }

    fn run_b(&self) -> String {
        let seed: Vec<usize> = self
            .input
            .trim()
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let mut cups = vec![0; 1_000_001];
        for (i, v) in seed.iter().enumerate() {
            cups[*v] = seed[(i + 1) % seed.len()];
        }
        for i in 11..1_000_001 {
            cups[i - 1] = i
        }
        cups[1_000_000] = seed[0];
        cups[*seed.last().unwrap()] = 10;
        let mut cur = seed[0];
        for _ in 0..10_000_000 {
            cur = round(&mut cups, cur);
        }
        (cups[1] * cups[cups[1]]).to_string()
    }
}

fn round(cups: &mut [usize], cur: usize) -> usize {
    let next = cups[cur];
    cups[cur] = cups[cups[cups[next]]];
    let mut tgt = if cur > 0 { cur - 1 } else { cups.len() - 1 };
    loop {
        let mut brk = true;
        if tgt == 0 {
            tgt = cups.len() - 1;
            brk = false;
        } else if tgt == next || tgt == cups[next] || tgt == cups[cups[next]] {
            tgt -= 1;
            brk = false;
        }
        if tgt == 0 {
            tgt = cups.len() - 1;
            brk = false;
        }
        if brk {
            break;
        }
    }
    cups[cups[cups[next]]] = cups[tgt];
    cups[tgt] = next;
    cups[cur]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "23"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "23_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("67384529"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("149245887792"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("72496583"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("41785843847"));
    }
}
