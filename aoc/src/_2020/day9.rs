use itertools::Itertools;

pub struct Runner {
    pub input: String,
    pub preamble_len: usize,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        if let Some(weakness) = find_weakness(&parse_input(&self.input), self.preamble_len) {
            return weakness.to_string();
        }
        String::from("Unable to find answer")
    }

    fn run_b(&self) -> String {
        let nums = parse_input(&self.input);
        if let Some(weakness) = find_weakness(&nums, self.preamble_len) {
            let mut i = 0;
            while nums[i] < weakness {
                let mut j = i + 1;
                while nums[j] < weakness {
                    if nums[i..j].iter().sum::<u64>() == weakness {
                        if let itertools::MinMaxResult::MinMax(min, max) =
                            nums[i..j].iter().minmax()
                        {
                            return (min + max).to_string();
                        }
                    }
                    j += 1;
                }
                i += 1;
            }
        }
        String::from("Unable to find answer")
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.parse()
                .unwrap_or_else(|_| panic!("Failed to parse: \"{}\"", l))
        })
        .collect()
}

fn find_weakness(nums: &[u64], premble_len: usize) -> Option<u64> {
    'outer: for vals in nums.windows(premble_len + 1) {
        let tgt = vals[premble_len];
        for pair in vals[0..premble_len].iter().combinations(2) {
            if pair[0] + pair[1] == tgt {
                continue 'outer;
            }
        }
        return Some(tgt);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "9"),
            preamble_len: 25,
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "9_simple"),
            preamble_len: 5,
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("127"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("62"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("466456641"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("55732936"));
    }
}
