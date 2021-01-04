extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let (rules, _, others) = self.parse_input();
        others
            .iter()
            .fold(0, |acc, ticket| {
                acc + ticket
                    .iter()
                    .filter(|v| rules.iter().all(|r| !r.valid(**v)))
                    .sum::<usize>()
            })
            .to_string()
    }

    fn run_b(&self) -> String {
        let (rules, mine, others) = self.parse_input();
        let valid: Vec<_> = others
            .iter()
            .filter(|ticket| {
                ticket
                    .iter()
                    .all(|val| rules.iter().any(|rule| rule.valid(*val)))
            })
            .collect();

        let mut possible = Vec::new();
        for pos in 0..20 {
            possible.push(HashSet::new());
            for rule in rules.iter() {
                if valid.iter().all(|ticket| rule.valid(ticket[pos])) {
                    possible[pos].insert(&rule.name);
                }
            }
        }

        let mut found = HashMap::new();
        possible
            .iter()
            .enumerate()
            .sorted_by_key(|(_, set)| set.len())
            .for_each(|(p, set)| {
                found.insert(set.iter().find(|v| !found.contains_key(v)).unwrap(), p);
            });

        found
            .iter()
            .fold(1, |acc, (field, pos)| {
                if field.starts_with("departure") {
                    acc * mine[*pos]
                } else {
                    acc
                }
            })
            .to_string()
    }
}

impl Runner {
    fn parse_input(&self) -> (Vec<Rule>, Vec<usize>, Vec<Vec<usize>>) {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new(r"(.+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }
        let (rulestr, minestr, otherstr) = self.input.split("\n\n").collect_tuple().unwrap();
        (
            rulestr
                .split('\n')
                .filter_map(|l| match RULE_RE.captures(l) {
                    Some(caps) => Some(Rule {
                        name: String::from(caps.get(1).unwrap().as_str()),
                        ranges: vec![
                            caps.get(2).unwrap().as_str().parse().unwrap(),
                            caps.get(3).unwrap().as_str().parse().unwrap(),
                            caps.get(4).unwrap().as_str().parse().unwrap(),
                            caps.get(5).unwrap().as_str().parse().unwrap(),
                        ],
                    }),
                    None => None,
                })
                .collect(),
            minestr
                .trim()
                .lines()
                .nth(1)
                .unwrap()
                .split(',')
                .map(|n| {
                    n.parse()
                        .unwrap_or_else(|e| panic!("Error parsing my ticket value {}: {}", n, e))
                })
                .collect(),
            otherstr
                .trim()
                .lines()
                .skip(1)
                .map(|l| {
                    l.split(',')
                        .map(|n| {
                            n.parse().unwrap_or_else(|e| {
                                panic!("Error parsing other ticket value {}: {}", n, e)
                            })
                        })
                        .collect()
                })
                .collect(),
        )
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<usize>,
}

impl Rule {
    fn valid(&self, tst: usize) -> bool {
        (self.ranges[0] <= tst && tst <= self.ranges[1])
            || (self.ranges[2] <= tst && tst <= self.ranges[3])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "16"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("23009"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("10458887314153"));
    }
}
