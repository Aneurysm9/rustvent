use std::collections::{HashMap, HashSet};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let wires = self.read_input();
        let origin = Point { x: 0, y: 0 };
        let mut min = std::i64::MAX;
        for intersection in wires.0[0].intersection(&wires.0[1]) {
            let dist = origin.distance(intersection);
            if dist < min && dist > 0 {
                min = dist;
            }
        }

        min.to_string()
    }

    fn run_b(&self) -> String {
        let wires = self.read_input();
        let mut min = std::usize::MAX;
        for intersection in wires.0[0].intersection(&wires.0[1]) {
            let dist =
                wires.1[0].get(intersection).unwrap() + wires.1[1].get(intersection).unwrap();
            if dist < min && dist > 0 {
                min = dist;
            }
        }

        min.to_string()
    }
}

impl Runner {
    fn read_input(&self) -> (Vec<HashSet<Point>>, Vec<HashMap<Point, usize>>) {
        let wires: Vec<Vec<Instruction>> = self
            .input
            .trim()
            .lines()
            .map(|l| l.split(',').map(|i| Instruction::new(i)).collect())
            .collect();
        let mut set = Vec::new();
        let mut map = Vec::new();
        for wire in wires {
            let mut points = HashSet::new();
            let mut distances = HashMap::new();
            let mut loc = Point { x: 0, y: 0 };
            let mut dist = 0;
            for instr in wire {
                loc = loc.apply(instr, &mut points, &mut distances, &mut dist);
            }
            set.push(points);
            map.push(distances)
        }

        (set, map)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn apply(
        &self,
        instr: Instruction,
        points: &mut HashSet<Point>,
        distances: &mut HashMap<Point, usize>,
        dist: &mut usize,
    ) -> Point {
        let mut cur = *self;
        let (step, mag) = match instr {
            Instruction::Left(m) => (Point { x: -1, y: 0 }, m),
            Instruction::Right(m) => (Point { x: 1, y: 0 }, m),
            Instruction::Up(m) => (Point { x: 0, y: -1 }, m),
            Instruction::Down(m) => (Point { x: 0, y: 1 }, m),
        };
        for _ in 0..mag {
            points.insert(cur);
            distances.entry(cur).or_insert(*dist);
            *dist += 1;
            cur = cur.add(&step)
        }
        cur
    }

    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
enum Instruction {
    Left(i64),
    Right(i64),
    Up(i64),
    Down(i64),
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let dir = input.chars().next().unwrap();
        let mag = input
            .trim_start_matches(char::is_alphabetic)
            .parse()
            .unwrap();
        match dir {
            'L' => Instruction::Left(mag),
            'R' => Instruction::Right(mag),
            'U' => Instruction::Up(mag),
            'D' => Instruction::Down(mag),
            d => panic!("Unexpected direction '{}' encountered", d),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "3"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("806"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("66076"));
    }
}
