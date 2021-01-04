pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let map: Vec<Vec<char>> = self.input.lines().map(|l| l.chars().collect()).collect();
        check_slope(&map, 1, 3).to_string()
    }

    fn run_b(&self) -> String {
        let map: Vec<Vec<char>> = self.input.lines().map(|l| l.chars().collect()).collect();
        let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
        let mut res = 1;
        for slope in slopes.iter() {
            let (rise, run) = slope;
            res *= check_slope(&map, *rise, *run);
        }
        res.to_string()
    }
}

fn check_slope(map: &[Vec<char>], rise: usize, run: usize) -> usize {
    let mut res = 0;
    let mut x = 0;
    let mut y = 0;
    let width = map[0].len();
    while y < map.len() {
        if map[y][x] == '#' {
            res += 1;
        }
        x = (x + run) % width;
        y += rise;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "3"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "3_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("7"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("336"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("200"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("3737923200"));
    }
}
