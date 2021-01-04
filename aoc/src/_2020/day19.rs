use regex::Regex;
use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let (rules, data) = self.parse_input();
        let re = Regex::new(&format!("^{}$", compile_rules(&rules, "0", 0))).unwrap();
        data.iter().filter(|l| re.is_match(l)).count().to_string()
    }

    fn run_b(&self) -> String {
        let (mut rules, data) = self.parse_input();
        rules.insert("8", "42 | 42 8");
        rules.insert("11", "42 31 | 42 11 31");
        let re = Regex::new(&format!("^{}$", compile_rules(&rules, "0", 0))).unwrap();
        data.iter().filter(|l| re.is_match(l)).count().to_string()
    }
}

impl Runner {
    fn parse_input(&self) -> (HashMap<&str, &str>, Vec<&str>) {
        let vals: Vec<_> = self.input.trim().split("\n\n").collect();
        (
            vals[0]
                .trim()
                .lines()
                .map(|l| {
                    let tmp: Vec<&str> = l.split(": ").collect();
                    (tmp[0], tmp[1])
                })
                .collect(),
            vals[1].trim().lines().collect(),
        )
    }
}

fn compile_rules(input: &HashMap<&str, &str>, tgt: &str, depth: usize) -> String {
    lazy_static! {
        static ref TERM_RE: Regex = Regex::new("^\"(.)\"$").unwrap();
        static ref SIMPLE_RE: Regex = Regex::new("^[\\d ]+$").unwrap();
    }

    if depth > 15 || tgt.is_empty() {
        return String::from("");
    }

    let tgt_rule = *input.get(tgt).unwrap();
    if TERM_RE.is_match(tgt_rule) {
        let caps = TERM_RE.captures(tgt_rule).unwrap();
        caps[1].into()
    } else if SIMPLE_RE.is_match(tgt_rule) {
        tgt_rule
            .split(' ')
            .map(|r| compile_rules(&input, r, depth + 1))
            .collect::<Vec<String>>()
            .join("")
    } else {
        format!(
            "({})",
            tgt_rule
                .split('|')
                .map(|a| a
                    .split(' ')
                    .map(|r| compile_rules(&input, r, depth + 1))
                    .collect::<Vec<String>>()
                    .join(""))
                .collect::<Vec<String>>()
                .join("|")
        )
    }
}
