use itertools::Itertools;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        find_weakness(&parse_input(&self.input)).to_string()
    }

    fn run_b(&self) -> String {
        let nums = parse_input(&self.input);
        let weakness = find_weakness(&nums);
        let mut i = 0;
        let mut res = 0;
        'outer: while nums[i] < weakness {
            let mut j = i + 1;
            while nums[j] < weakness {
                let iter = nums[i..j].iter();
                if iter.clone().sum::<u64>() == weakness {
                    let min = iter.clone().min().unwrap();
                    let max = iter.max().unwrap();
                    res = min + max;
                    break 'outer;
                }
                j += 1;
            }
            i += 1;
        }
        res.to_string()
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .lines()
        .map(|l| l.parse().expect(&(format!("Failed to parse: \"{}\"", l))))
        .collect()
}

fn find_weakness(nums: &Vec<u64>) -> u64 {
    let mut res = 0;
    'outer: for vals in nums.windows(26) {
        let tgt = vals[25];
        for pair in vals[0..25].iter().combinations(2) {
            if pair[0] + pair[1] == tgt {
                continue 'outer;
            }
        }
        res = tgt;
    }
    res
}
