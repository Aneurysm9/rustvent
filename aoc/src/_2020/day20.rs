use itertools::Itertools;
use std::{collections::HashMap, fmt, ops::Deref};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        match_tiles(&self.parse_input())
            .iter()
            .filter_map(|(id, c)| if c.len() == 2 { Some(id) } else { None })
            .product::<usize>()
            .to_string()
    }

    fn run_b(&self) -> String {
        // let tiles = self.parse_input();
        // let matches = match_tiles(&tiles);
        String::from("Not implemented")
    }
}

impl Runner {
    fn parse_input(&self) -> HashMap<usize, Tile> {
        self.input
            .trim()
            .split("\n\n")
            .map(|t| {
                let lines: Vec<_> = t.trim().lines().collect();
                (
                    lines[0]
                        .trim()
                        .split(' ')
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(':')
                        .parse()
                        .unwrap(),
                    Tile(
                        lines
                            .iter()
                            .skip(1)
                            .enumerate()
                            .flat_map(|(y, l)| {
                                l.trim()
                                    .chars()
                                    .enumerate()
                                    .map(|(x, c)| (Point(x, y), c == '#'))
                                    .collect::<Vec<_>>()
                            })
                            .collect(),
                    ),
                )
            })
            .collect()
    }
}

fn match_tiles(tiles: &HashMap<usize, Tile>) -> HashMap<usize, Vec<usize>> {
    let mut matches = HashMap::<usize, Vec<usize>>::new();

    for (a, b) in tiles.iter().tuple_combinations() {
        if a.1.compare(b.1) {
            let mut va = match matches.get(a.0) {
                Some(v) => v.clone(),
                None => Vec::<usize>::new(),
            };
            va.push(*b.0);
            matches.insert(*a.0, va);

            let mut vb = match matches.get(b.0) {
                Some(v) => v.clone(),
                None => Vec::<usize>::new(),
            };
            vb.push(*a.0);
            matches.insert(*b.0, vb);
        }
    }
    matches
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point(usize, usize);

impl Point {
    fn mul(&self, other: &Point) -> Point {
        Point(self.0 * other.0, self.1 * other.1)
    }

    fn add(&self, other: &Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

type Cell = bool;

#[derive(Debug, PartialEq, Eq)]
struct Tile(HashMap<Point, Cell>);

impl Tile {
    fn side_len(&self) -> usize {
        (self.len() as f64).sqrt().floor() as usize
    }

    fn side_end(&self) -> usize {
        self.side_len() - 1
    }

    #[allow(unused)]
    fn rotr(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(self.side_end() - p.1, p.0), *c);
        }
        Tile(out)
    }

    #[allow(unused)]
    fn flipv(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(p.0, self.side_end() - p.1), *c);
        }
        Tile(out)
    }

    #[allow(unused)]
    fn fliph(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(self.side_end() - p.0, p.1), *c);
        }
        Tile(out)
    }

    fn side(&self, start: Point, offset: Point) -> Side {
        let mut out = Vec::new();
        for i in 0..self.side_len() {
            out.push(*self.get(&start.add(&offset.mul(&Point(i, i)))).unwrap())
        }
        Side(out)
    }

    fn left(&self) -> Side {
        self.side(Point(0, 0), Point(0, 1))
    }

    fn right(&self) -> Side {
        self.side(Point(self.side_end(), 0), Point(0, 1))
    }

    fn top(&self) -> Side {
        self.side(Point(0, 0), Point(1, 0))
    }

    fn bottom(&self) -> Side {
        self.side(Point(0, self.side_end()), Point(1, 0))
    }

    fn sides(&self) -> Vec<Side> {
        vec![
            self.left(),
            self.right(),
            self.top(),
            self.bottom(),
            self.left().reverse(),
            self.right().reverse(),
            self.top().reverse(),
            self.bottom().reverse(),
        ]
    }

    fn compare(&self, other: &Tile) -> bool {
        for a in self.sides() {
            for b in other.sides() {
                if a == b {
                    return true;
                }
            }
        }
        false
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.side_len();
        for y in 0..len {
            for x in 0..len {
                if let Err(e) = write!(
                    f,
                    "{}",
                    if *self.get(&Point(x, y)).unwrap() {
                        '#'
                    } else {
                        '.'
                    }
                ) {
                    return Err(e);
                }
            }
            if y < len - 1 {
                if let Err(e) = writeln!(f) {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}

impl Deref for Tile {
    type Target = HashMap<Point, Cell>;
    fn deref(&self) -> &HashMap<Point, Cell> {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Side(Vec<Cell>);

impl Side {
    fn reverse(self) -> Side {
        let mut out = self.clone();
        out.reverse();
        Side(out)
    }
}

impl Deref for Side {
    type Target = Vec<Cell>;
    fn deref(&self) -> &Vec<Cell> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "20"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "20_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("20899048083289"));
    }

    // #[test]
    // fn simple_b() {
    //     assert_eq!(simple().run_b(), String::from("273"));
    // }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("16192267830719"));
    }

    // #[test]
    // fn real_b() {
    //     assert_eq!(new().run_b(), String::from("1909"));
    // }
}
