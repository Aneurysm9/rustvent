extern crate regex;

use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        Grid::new(&self.input, false).nth(5).unwrap().to_string()
    }

    fn run_b(&self) -> String {
        Grid::new(&self.input, true).nth(5).unwrap().to_string()
    }
}

impl Runner {}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}

impl Point {
    fn neighbors(&self, part_b: bool) -> Vec<Point> {
        let mut res = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                for z in -1..2 {
                    if part_b {
                        for w in &[-1, 1] {
                            if self.x == self.x + x
                                && self.y == self.y + y
                                && self.z == self.z + z
                                && self.w == self.w + w
                            {
                                continue;
                            }
                            res.push(Point {
                                x: self.x + x,
                                y: self.y + y,
                                z: self.z + z,
                                w: self.w + w,
                            });
                        }
                    }
                    if self.x == self.x + x && self.y == self.y + y && self.z == self.z + z {
                        continue;
                    }
                    res.push(Point {
                        x: self.x + x,
                        y: self.y + y,
                        z: self.z + z,
                        w: self.w,
                    });
                }
            }
        }
        res
    }
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<Point, bool>,
    part_b: bool,
}

impl Grid {
    fn new(input: &str, part_b: bool) -> Grid {
        let mut cells = HashMap::new();
        input.trim().lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    cells.insert(
                        Point {
                            x: x as i32,
                            y: y as i32,
                            z: 0,
                            w: 0,
                        },
                        true,
                    );
                }
            })
        });
        Grid { cells, part_b }
    }
}

impl Iterator for Grid {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cells = HashMap::new();
        let mut secondary = Vec::new();
        for (c, active_cell) in self.cells.iter() {
            let mut active = 0;
            for n in c.neighbors(self.part_b) {
                match self.cells.get(&n) {
                    Some(v) => {
                        if *v {
                            active += 1;
                        }
                    }
                    None => secondary.push(n),
                }
            }
            if active == 3 || *active_cell && active == 2 {
                cells.insert(*c, true);
            }
        }
        for c in secondary {
            let mut active = 0;
            for n in c.neighbors(self.part_b) {
                if let Some(v) = self.cells.get(&n) {
                    if *v {
                        active += 1;
                    }
                }
            }
            if active == 3 {
                cells.insert(c, true);
            }
        }
        self.cells = cells;
        Some(self.cells.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "17"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "17_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("112"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("848"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("353"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("2472"));
    }
}
