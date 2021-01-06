use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt,
    hash::Hash,
    ops::{Deref, DerefMut},
};

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
        let mut big = assemble_puzzle(&mut self.parse_input()).rescale();
        let nessie = Tile::nessie();

        for _ in 0..4 {
            for _ in 0..2 {
                let count = big.nessie_count(&nessie);
                if count > 0 {
                    return (big.rough_count() - nessie.rough_count() * count).to_string();
                }
                big = big.fliph();
            }
            big = big.rotr();
        }
        String::from("Error encountered")
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
                                    .map(|(x, c)| (Point(x as isize, y as isize), c == '#'))
                                    .collect::<Vec<_>>()
                            })
                            .collect(),
                    ),
                )
            })
            .collect()
    }
}

fn assemble_puzzle(tiles: &mut HashMap<usize, Tile>) -> Tile {
    let mut big = Tile(HashMap::new());
    let matches = match_tiles(tiles);

    let initial = matches.iter().next().unwrap();

    let mut corners = HashMap::new();
    corners.insert(initial.0, Point(1000, 1000));
    big.merge(&tiles.get(initial.0).unwrap().trim(), Point(1000, 1000));

    let mut frontier = vec![initial];
    let mut seen = HashSet::new();

    while !frontier.is_empty() {
        let (id, others) = frontier.pop().unwrap();
        if seen.contains(id) {
            continue;
        }
        seen.insert(id);
        let tile = tiles.get(&id).unwrap().clone();
        let start = corners.get(&id).unwrap();
        let side_len = tile.side_len() - 2;
        let sides = &[
            ("top", start.add(&Point(0, -side_len))),
            ("left", start.add(&Point(-side_len, 0))),
            ("right", start.add(&Point(side_len, 0))),
            ("bottom", start.add(&Point(0, side_len))),
        ];
        for oid in others {
            if seen.contains(oid) {
                continue;
            }
            frontier.push((oid, matches.get(oid).unwrap()));
            for (side, offset) in sides {
                if let Some(aligned) = tile.align(side, tiles.get(&oid).unwrap()) {
                    big.merge(&aligned.trim(), *offset);
                    corners.insert(oid, *offset);
                    tiles.insert(*oid, aligned);
                    break;
                }
            }
        }
    }
    big
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point(isize, isize);

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

impl Clone for Tile {
    fn clone(&self) -> Self {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(*p, *c);
        }
        Self(out)
    }
}

impl Tile {
    fn nessie() -> Tile {
        Tile(
            String::from(
                "..................#.
#....##....##....###
.#..#..#..#..#..#...",
            )
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.trim()
                    .chars()
                    .enumerate()
                    .map(|(x, c)| (Point(x as isize, y as isize), c == '#'))
                    .collect::<Vec<_>>()
            })
            .collect(),
        )
    }

    fn side_len(&self) -> isize {
        (self.len() as f64).sqrt().floor() as isize
    }

    fn side_end(&self) -> isize {
        self.side_len() - 1
    }

    fn rotr(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(self.side_end() - p.1, p.0), *c);
        }
        Tile(out)
    }

    fn flipv(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(p.0, self.side_end() - p.1), *c);
        }
        Tile(out)
    }

    fn fliph(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            out.insert(Point(self.side_end() - p.0, p.1), *c);
        }
        Tile(out)
    }

    fn trim(&self) -> Tile {
        let mut out = HashMap::<Point, Cell>::new();
        for (p, c) in self.iter() {
            if p.0 == 0 || p.1 == 0 || p.0 == self.side_end() || p.1 == self.side_end() {
                continue;
            }
            out.insert(Point(p.0 - 1, p.1 - 1), *c);
        }
        Tile(out)
    }

    fn merge(&mut self, t2: &Tile, offset: Point) {
        for (p, c) in t2.iter() {
            self.insert(p.add(&offset), *c);
        }
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

    fn align(&self, side: &str, other: &Tile) -> Option<Tile> {
        let tgt = match side {
            "top" => self.top(),
            "left" => self.left(),
            "right" => self.right(),
            "bottom" => self.bottom(),
            s => panic!("Unexpected side match attempted: {}", s),
        };
        let mut ret = other.clone();
        for _ in 0..4 {
            let cmp = match side {
                "top" => ret.bottom(),
                "left" => ret.right(),
                "right" => ret.left(),
                "bottom" => ret.top(),
                s => panic!("Unexpected side match attempted: {}", s),
            };

            if tgt == cmp {
                return Some(ret);
            } else if tgt == cmp.reverse() {
                return match side {
                    "top" => Some(ret.fliph()),
                    "left" => Some(ret.flipv()),
                    "right" => Some(ret.flipv()),
                    "bottom" => Some(ret.fliph()),
                    s => panic!("Unexpected side match attempted: {}", s),
                };
            }
            ret = ret.rotr();
        }
        None
    }

    fn is_at(&self, other: &Tile, offset: Point) -> bool {
        for (p, c) in self.iter() {
            let p2 = p.add(&offset);
            if !other.contains_key(&p2) {
                return false;
            }
            if *c && !other.get(&p2).unwrap() {
                return false;
            }
        }
        true
    }

    fn nessie_count(&self, nessie: &Tile) -> usize {
        self.keys().filter(|p| nessie.is_at(self, **p)).count()
    }

    fn rough_count(&self) -> usize {
        self.values().filter(|v| **v).count()
    }

    fn rescale(&self) -> Tile {
        let minx = self.keys().map(|p| p.0).min().unwrap();
        let miny = self.keys().map(|p| p.1).min().unwrap();
        let offset = Point(-minx, -miny);
        Tile(self.iter().map(|(p, c)| (p.add(&offset), *c)).collect())
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
                    if !self.contains_key(&Point(x, y)) {
                        'X'
                    } else if *self.get(&Point(x, y)).unwrap() {
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

impl DerefMut for Tile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("273"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("16192267830719"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("1909"));
    }
}
