use itertools::Itertools;
use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

#[derive(Debug, Clone)]
struct Bus {
    id: i64,
    offset: i64,
}
#[derive(Debug, Clone)]
struct Contest {
    earliest: i64,
    busses: HashMap<i64, Bus>,
}

fn parse_input(input: &str) -> Contest {
    let lines: Vec<&str> = input.trim().lines().collect();
    Contest {
        earliest: lines[0]
            .parse()
            .unwrap_or_else(|e| panic!("Unable to parse timestamp {}: {}", lines[0], e)),
        busses: lines[1]
            .split(',')
            .enumerate()
            .filter_map(|(i, v)| {
                if v == "x" {
                    return None;
                }
                let id: i64 = v
                    .parse()
                    .unwrap_or_else(|e| panic!("Unable to parse ID {}: {}", v, e));
                Some((
                    id,
                    Bus {
                        id,
                        offset: i as i64,
                    },
                ))
            })
            .collect(),
    }
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let contest = parse_input(&self.input);
        let (bus, wait) = contest
            .busses
            .iter()
            .map(|(id, _)| (*id, (*id - (contest.earliest % *id)).abs()))
            .min_by_key(|(_, delta)| *delta)
            .unwrap();
        (bus * wait).to_string()
    }

    fn run_b(&self) -> String {
        let busses = parse_input(&self.input).busses;
        let ids: Vec<&i64> = busses.keys().sorted_by_key(|v| -*v).collect();
        let mut ptr = 0;
        let mut skip = *ids[ptr];
        let mut start = skip - busses.get(ids[ptr]).unwrap().offset;
        ptr += 1;
        while ptr < ids.len() {
            let bus = busses.get(ids[ptr]).unwrap();
            while (start + bus.offset) % bus.id != 0 {
                start += skip;
            }
            skip *= ids[ptr];
            ptr += 1;
        }
        start.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "13"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "13_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("295"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("1068781"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("115"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("756261495958122"));
    }
}
