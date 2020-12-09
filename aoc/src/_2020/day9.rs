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
        while nums[i] < weakness {
            let mut j = i + 1;
            while nums[j] < weakness {
                if nums[i..j].iter().sum::<u64>() == weakness {
                    if let itertools::MinMaxResult::MinMax(min, max) = nums[i..j].iter().minmax() {
                        return (min + max).to_string();
                    }
                }
                j += 1;
            }
            i += 1;
        }
        String::from("Unable to find answer")
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
    'outer: for vals in nums.windows(26) {
        let tgt = vals[25];
        for pair in vals[0..25].iter().combinations(2) {
            if pair[0] + pair[1] == tgt {
                continue 'outer;
            }
        }
        return tgt;
    }
    0
}
