pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut res = 0;
        for line in self.input.lines() {
            let vals: Vec<&str> = line.split(' ').collect();
            let range: Vec<usize> = vals[0]
                .trim()
                .split('-')
                .map(|v| v.trim().parse().expect("Unable to parse range"))
                .collect();
            let restr: char = vals[1].chars().next().unwrap();
            let count: usize = vals[2].trim().chars().filter(|c| c == &restr).count();
            if count >= range[0] && count <= range[1] {
                res += 1;
            }
        }
        res.to_string()
    }

    fn run_b(&self) -> String {
        let mut res = 0;
        for line in self.input.lines() {
            let vals: Vec<&str> = line.split(' ').collect();
            let range: Vec<usize> = vals[0]
                .trim()
                .split('-')
                .map(|v| v.trim().parse().expect("Unable to parse range"))
                .collect();
            let restr: char = vals[1].chars().next().unwrap();
            let chars: Vec<char> = vals[2].trim().chars().collect();
            if (chars[range[0] - 1] == restr) ^ (chars[range[1] - 1] == restr) {
                res += 1;
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
            input: read_input(2020, "2"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "2_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("2"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("1"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("410"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("694"));
    }
}
