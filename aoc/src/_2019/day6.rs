use std::collections::{HashMap, HashSet};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let c = self.parse_input();
        c.objects
            .keys()
            .map(|id| c.orbit_count(id.to_string()))
            .sum::<usize>()
            .to_string()
    }

    fn run_b(&self) -> String {
        let c = self.parse_input();
        let mut mine = HashSet::new();
        let mut tgt = String::from("YOU");
        while let Some(cur) = c.objects.get(&tgt) {
            mine.insert(cur.id.to_string());
            tgt = cur.parent.to_string();
        }

        tgt = String::from("SAN");
        while let Some(cur) = c.objects.get(&tgt) {
            if mine.contains(&cur.id) {
                break;
            }
            tgt = cur.parent.to_string();
        }

        (c.distance("YOU".to_string(), tgt.to_string()) + c.distance("SAN".to_string(), tgt))
            .to_string()
    }
}

impl Runner {
    fn parse_input(&self) -> Constellation {
        Constellation {
            objects: self
                .input
                .trim()
                .lines()
                .map(|l| {
                    let parts: Vec<_> = l.split(')').collect();
                    (
                        parts[1].to_string(),
                        Object {
                            id: parts[1].to_string(),
                            parent: parts[0].to_string(),
                        },
                    )
                })
                .collect(),
        }
    }
}

#[derive(Debug)]
struct Constellation {
    objects: HashMap<String, Object>,
}

impl Constellation {
    fn orbit_count(&self, id: String) -> usize {
        let mut count = 0;
        let mut tgt = id;
        while let Some(cur) = self.objects.get(&tgt) {
            count += 1;
            tgt = cur.parent.to_string();
        }
        count
    }

    fn distance(&self, from: String, to: String) -> usize {
        let mut dist = 0;
        let mut tgt = from;
        while let Some(cur) = self.objects.get(&tgt) {
            tgt = cur.parent.to_string();
            if tgt == to {
                break;
            }
            dist += 1;
        }
        dist
    }
}

#[derive(Debug)]
struct Object {
    id: String,
    parent: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "6"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("312697"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("466"));
    }
}
