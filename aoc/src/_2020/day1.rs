use itertools::Itertools;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut res = 0;
        for pairs in self
            .input
            .lines()
            .map(|l| l.trim().parse().expect("Unable to parse input"))
            .combinations(2)
        {
            let i: u64 = pairs[0];
            let j: u64 = pairs[1];
            if i + j == 2020 {
                res = i * j
            }
        }
        res.to_string()
    }

    fn run_b(&self) -> String {
        let mut res = 0;
        for pairs in self
            .input
            .lines()
            .map(|l| l.trim().parse().expect("Unable to parse input"))
            .combinations(3)
        {
            let i: u64 = pairs[0];
            let j: u64 = pairs[1];
            let k: u64 = pairs[2];
            if i + j + k == 2020 {
                res = i * j * k
            }
        }
        res.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "1"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "1_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("514579"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("241861950"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("1007331"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("48914340"));
    }
}
