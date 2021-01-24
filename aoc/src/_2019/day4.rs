pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        PasswordIter::new(&self.input, false).count().to_string()
    }

    fn run_b(&self) -> String {
        PasswordIter::new(&self.input, true).count().to_string()
    }
}

struct PasswordIter {
    cur: usize,
    max: usize,
    part_b: bool,
}

impl PasswordIter {
    fn new(range: &str, part_b: bool) -> PasswordIter {
        let r: Vec<_> = range.trim().split('-').collect();
        PasswordIter {
            cur: r[0].parse::<usize>().unwrap() - 1,
            max: r[1].parse().unwrap(),
            part_b,
        }
    }
}

impl Iterator for PasswordIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.cur == self.max {
            return None;
        }
        self.cur += 1;

        while !is_valid(self.cur, self.part_b) {
            if self.cur >= self.max {
                return None;
            }
            self.cur += 1;
        }

        Some(self.cur)
    }
}

fn is_valid(cur: usize, part_b: bool) -> bool {
    let mut good_double = false;
    let mut double_len = 1;
    let mut last = cur % 10;
    let mut tmp = cur / 10;

    while tmp > 0 {
        let next = tmp % 10;
        match last.cmp(&next) {
            std::cmp::Ordering::Equal => {
                if !part_b {
                    good_double = true
                } else {
                    double_len += 1
                }
            }
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Greater => {
                if part_b && double_len == 2 {
                    good_double = true
                }
                double_len = 1;
            }
        }
        last = next;
        tmp /= 10;
    }

    good_double || double_len == 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "4"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("1330"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("876"));
    }

    #[test]
    fn valid_a() {
        assert_eq!(is_valid(111111, false), true);
        assert_eq!(is_valid(223450, false), false);
        assert_eq!(is_valid(123789, false), false);
    }

    #[test]
    fn valid_b() {
        assert_eq!(is_valid(112233, true), true);
        assert_eq!(is_valid(123444, true), false);
        assert_eq!(is_valid(111122, true), true);
        assert_eq!(is_valid(112222, true), true);
    }
}
