use std::fmt;
pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.parse_input()
            .iter()
            .map(|v| (v.count(0), v.count(1) * v.count(2)))
            .min_by(|x, y| x.0.cmp(&y.0))
            .unwrap()
            .1
            .to_string()
    }

    fn run_b(&self) -> String {
        let layers = self.parse_input();
        let mut res = Layer::new();
        let mut l1 = layers.first().unwrap();
        for l2 in layers.iter().skip(1) {
            res = l1.combine(l2).unwrap();
            l1 = &res;
        }
        format!("{}", res)
    }
}

impl Runner {
    fn parse_input(&self) -> Vec<Layer> {
        let mut res = Vec::new();
        let mut cur = Layer::new();
        let mut tmp_row = Vec::new();
        for (idx, pixel) in self
            .input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .enumerate()
        {
            if idx > 0 && idx % 25 == 0 {
                cur.pixels.push(tmp_row);
                tmp_row = Vec::new();
                if idx % (25 * 6) == 0 {
                    res.push(cur);
                    cur = Layer::new();
                }
            }
            tmp_row.push(pixel);
        }
        cur.pixels.push(tmp_row);
        res.push(cur);
        res
    }
}

struct Layer {
    pixels: Vec<Vec<u8>>,
}

impl Layer {
    fn new() -> Layer {
        Layer { pixels: Vec::new() }
    }

    #[allow(clippy::clippy::naive_bytecount)]
    fn count(&self, tgt: u8) -> usize {
        self.pixels
            .iter()
            .map(|r| r.iter().filter(|i| **i == tgt).count())
            .sum()
    }

    fn combine(&self, other: &Layer) -> Option<Layer> {
        if self.pixels.len() != other.pixels.len() {
            return None;
        }

        let mut res = Layer::new();
        for (ri, row) in self.pixels.iter().enumerate() {
            res.pixels.push(Vec::new());
            for (pi, p) in row.iter().enumerate() {
                match p {
                    0..=1 => res.pixels.last_mut().unwrap().push(*p),
                    2 => res.pixels.last_mut().unwrap().push(other.pixels[ri][pi]),
                    _ => panic!("Oh noes!"),
                }
            }
        }
        Some(res)
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.pixels
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|p| match p {
                            0 => '.',
                            1 => '#',
                            _ => panic!("Wat!"),
                        })
                        .collect::<String>()
                        + "\n"
                })
                .collect::<String>()
                .trim()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "8"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("1572"));
    }

    #[test]
    fn real_b() {
        assert_eq!(
            new().run_b(),
            String::from(
                "#..#.#...##..#.####.####.
#.#..#...##..#.#....#....
##....#.#.####.###..###..
#.#....#..#..#.#....#....
#.#....#..#..#.#....#....
#..#...#..#..#.#....####."
            )
        );
    }
}
