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
    use crate::Solution;

    #[test]
    fn simple_a() {
        assert_eq!(
            Runner {
                input: String::from(
                    "1721
979
366
299
675
1456"
                )
            }
            .run_a(),
            String::from("514579")
        );
    }

    #[test]
    fn simple_b() {
        assert_eq!(
            Runner {
                input: String::from(
                    "1721
979
366
299
675
1456"
                )
            }
            .run_b(),
            String::from("241861950")
        );
    }
}
