use std::collections::HashMap;

use itertools::Itertools;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut ones = 1;
        let mut threes = 1;
        let vals: Vec<_> = self
            .input
            .trim()
            .lines()
            .map(|l| {
                l.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Unable to parse input: {}", l))
            })
            .sorted()
            .collect();
        vals.windows(2).for_each(|v| match v[1] - v[0] {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        });
        (ones * threes).to_string()
    }

    fn run_b(&self) -> String {
        let vals: Vec<_> = self
            .input
            .trim()
            .lines()
            .map(|l| {
                l.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Unable to parse input: {}", l))
            })
            .sorted()
            .collect();
        let exists: HashMap<&usize, usize> = vals.iter().enumerate().map(|(k, v)| (v, k)).collect();
        let mut map: HashMap<usize, usize> = HashMap::new();
        let last = vals.len() - 1;
        map.insert(last, 1);
        for (k, val) in vals.iter().rev().skip(1).enumerate() {
            let mut sum = 0;
            for diff in 1..4 {
                if let Some(cur) = exists.get(&(val + diff)) {
                    if let Some(next) = map.get(cur) {
                        sum += next;
                    }
                }
            }
            map.insert(last - k - 1, sum);
        }

        let mut res = 0;
        for v in 1..4 {
            if let Some(cur) = exists.get(&v) {
                res += map.get(cur).unwrap_or(&0);
            }
        }

        res.to_string()
    }
}
