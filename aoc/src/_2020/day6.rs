use itertools::Itertools;
use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.input
            .split("\n\n")
            .map(|l| l.chars().filter(|c| *c != '\n').unique().count())
            .sum::<usize>()
            .to_string()
    }

    fn run_b(&self) -> String {
        self.input
            .split("\n\n")
            .map(|l| count_answers(l))
            .sum::<usize>()
            .to_string()
    }
}

fn count_answers(set: &str) -> usize {
    let count = set.lines().count();
    let mut seen = HashMap::new();
    for line in set.lines() {
        for c in line.chars() {
            if let Some(v) = seen.get_mut(&c) {
                *v += 1;
            } else {
                seen.insert(c, 1);
            }
        }
    }
    seen.values().filter(|&v| *v == count).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "6"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "6_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("11"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("6"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("7283"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("3520"));
    }
}
