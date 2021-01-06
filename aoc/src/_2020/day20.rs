use itertools::Itertools;
use std::{
    collections::HashMap,
    fmt,
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
        let tiles = self.parse_input();
        let matches = match_tiles(&tiles);
        let side_len = (matches.len() as f64).sqrt().floor() as usize;
        let (mut start, mut next) = matches.iter().find(|(_, v)| v.len() == 2).unwrap();
        let mut places: Vec<Vec<(usize, Tile)>> = Vec::new();
        places.push(Vec::new());
        places[0].push((*start, tiles.get(start).unwrap().clone()));
        let (mut x, mut y) = (1, 0);
        loop {
            while places[y].len() < side_len {
                let mut matched = false;
                while !matched {
                    'm2: for id in next.iter() {
                        let mut t2 = tiles[id].clone();
                        for _ in 0..4 {
                            if places[y][x - 1].1.right() == t2.left() {
                                places[y].push((*id, t2));
                                matched = true;
                                start = id;
                                next = &matches[start];
                                break 'm2;
                            }
                            if places[y][x - 1].1.right() == t2.left().reverse() {
                                places[y].push((*id, t2.flipv()));
                                matched = true;
                                start = id;
                                next = &matches[start];
                                break 'm2;
                            }
                            t2 = t2.rotr();
                        }
                    }
                    if !matched && y == 0 {
                        places[y][0].1 = places[y][0].1.rotr();
                    }
                }
                x += 1;
            }

            x = 0;
            y += 1;
            if y == side_len {
                break;
            }
            places.push(Vec::new());
            start = &places[y - 1][x].0;
            next = &matches[start];
            let mut matched = false;
            while !matched {
                'm: for id in next.iter() {
                    let mut t2 = tiles[id].clone();
                    for _ in 0..4 {
                        if places[y - 1][x].1.bottom() == t2.top() {
                            places[y].push((*id, t2));
                            matched = true;
                            start = id;
                            next = &matches[start];
                            break 'm;
                        }
                        if places[y - 1][x].1.bottom() == t2.top().reverse() {
                            places[y].push((*id, t2.fliph()));
                            matched = true;
                            start = id;
                            next = &matches[start];
                            break 'm;
                        }
                        t2 = t2.rotr();
                    }
                }
                if !matched && y == 1 {
                    places[y - 1] = places[y - 1].iter().map(|t| (t.0, t.1.rotr())).collect();
                }
            }
            x += 1;
        }

        let mut big = Tile(HashMap::new());
        let mul = places[0][0].1.side_end() - 1;
        for (r, rv) in places.iter().enumerate() {
            for (c, (_, t)) in rv.iter().enumerate() {
                big.merge(&t.trim(), Point(c * mul, r * mul));
            }
        }
        println!("{}", big.side_len());
        let nessie = Tile::nessie();
        for _ in 0..4 {
            for _ in 0..2 {
                let count = big.nessie_count(&nessie);
                if count > 0 {
                    println!("{}", big);
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

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
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
                    .map(|(x, c)| (Point(x, y), c == '#'))
                    .collect::<Vec<_>>()
            })
            .collect(),
        )
    }

    fn side_len(&self) -> usize {
        (self.len() as f64).sqrt().floor() as usize
    }

    fn side_end(&self) -> usize {
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
