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
                seen.insert(c.clone(), 1);
            }
        }
    }
    seen.values().filter(|&v| *v == count).count()
}
