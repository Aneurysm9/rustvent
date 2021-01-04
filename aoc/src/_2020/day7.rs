use regex::Regex;
use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

static TARGET: &str = "shiny gold";

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let bags: HashMap<_, _> = self
            .input
            .lines()
            .map(|l| parse_bag(String::from(l)))
            .collect();
        let mut res = 0;
        for name in bags.keys() {
            if contains(&bags, name, TARGET) {
                res += 1;
            }
        }
        res.to_string()
    }

    fn run_b(&self) -> String {
        let bags: HashMap<_, _> = self
            .input
            .lines()
            .map(|l| parse_bag(String::from(l)))
            .collect();
        inner_count(&bags, TARGET).to_string()
    }
}

fn parse_bag(line: String) -> (String, HashMap<String, usize>) {
    lazy_static! {
        static ref BAG_RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.*)\.$").unwrap();
        static ref CONTENTS_RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    }
    let caps = BAG_RE.captures_iter(&line).next().unwrap();
    let mut bag = HashMap::new();
    for inner in CONTENTS_RE.captures_iter(&caps[2]) {
        let count = &inner[1].parse().expect("Unable to parse bag count");
        bag.insert((&inner[2]).to_string(), *count);
    }
    (caps[1].to_owned(), bag)
}

fn contains(bags: &HashMap<String, HashMap<String, usize>>, start: &str, target: &str) -> bool {
    if let Some(bag) = bags.get(start) {
        if bag.contains_key(target) {
            return true;
        }
        for name in bag.keys() {
            if contains(bags, name, target) {
                return true;
            }
        }
    }
    false
}

fn inner_count(bags: &HashMap<String, HashMap<String, usize>>, start: &str) -> usize {
    let mut res = 0;
    if let Some(bag) = bags.get(start) {
        for (name, count) in bag.iter() {
            res += count;
            res += count * inner_count(bags, name);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "7"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "7_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("4"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("32"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("185"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("89084"));
    }
}
