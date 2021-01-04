use std::collections::{hash_map::Entry, HashMap, HashSet};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.init_tiles()
            .values()
            .filter(|v| **v)
            .count()
            .to_string()
    }

    fn run_b(&self) -> String {
        let mut tiles = self.init_tiles();
        for _ in 0..100 {
            let mut new_tiles = HashMap::<Location, bool>::new();
            let mut frontier = HashSet::<Location>::new();
            for (k, v) in tiles.iter() {
                let count = k.neighbors().iter().fold(0, |acc, n| {
                    acc + if tiles.contains_key(n) && *tiles.get(n).unwrap() {
                        1
                    } else {
                        if !tiles.contains_key(n) {
                            frontier.insert(*n);
                        }
                        0
                    }
                });
                if *v && (count == 0 || count > 2) {
                    new_tiles.insert(*k, false);
                } else if !v && count == 2 {
                    new_tiles.insert(*k, true);
                } else {
                    new_tiles.insert(*k, *v);
                }
            }
            for k in frontier.iter() {
                let count = k.neighbors().iter().fold(0, |acc, n| {
                    acc + if tiles.contains_key(n) && *tiles.get(n).unwrap() {
                        1
                    } else {
                        0
                    }
                });
                if count == 2 {
                    new_tiles.insert(*k, true);
                }
            }
            tiles = new_tiles;
        }
        tiles.values().filter(|v| **v).count().to_string()
    }
}

impl Runner {
    fn parse_input(&self) -> impl Iterator<Item = Vec<Dir>> + '_ {
        self.input.trim().lines().map(|l| {
            let mut iter = l.chars();
            let mut res = Vec::new();
            while let Some(cur) = iter.next() {
                match cur {
                    'e' => res.push(Dir::East),
                    'w' => res.push(Dir::West),
                    'n' => match iter.next() {
                        Some('e') => res.push(Dir::Northeast),
                        Some('w') => res.push(Dir::Northwest),
                        Some(e) => panic!("Unexpected value found: {}", e),
                        None => panic!("Unexpected end of line"),
                    },
                    's' => match iter.next() {
                        Some('e') => res.push(Dir::Southeast),
                        Some('w') => res.push(Dir::Southwest),
                        Some(e) => panic!("Unexpected value found: {}", e),
                        None => panic!("Unexpected end of line"),
                    },
                    e => panic!("Unexpected character found: {}", e),
                }
            }
            res
        })
    }

    fn init_tiles(&self) -> HashMap<Location, bool> {
        let mut tiles = HashMap::<Location, bool>::new();
        for instr in self.parse_input() {
            let mut loc = Location::new();
            for dir in instr.iter() {
                loc.step(dir);
            }
            match tiles.entry(loc) {
                Entry::Occupied(mut o) => o.insert(!o.get()),
                Entry::Vacant(v) => *v.insert(true),
            };
        }

        tiles
    }
}

enum Dir {
    Northeast,
    Northwest,
    East,
    West,
    Southeast,
    Southwest,
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Location {
        Location { x: 0, y: 0 }
    }

    fn step(&mut self, d: &Dir) {
        match d {
            Dir::Northeast => {
                self.x += 1;
                self.y -= 1;
            }
            Dir::Northwest => self.y -= 1,
            Dir::East => self.x += 1,
            Dir::West => self.x -= 1,
            Dir::Southeast => self.y += 1,
            Dir::Southwest => {
                self.x -= 1;
                self.y += 1;
            }
        }
    }

    fn neighbors(self) -> Vec<Location> {
        vec![(1, -1), (0, -1), (1, 0), (-1, 0), (0, 1), (-1, 1)]
            .iter()
            .map(|(x, y)| Location {
                x: self.x + x,
                y: self.y + y,
            })
            .collect()
    }
}
