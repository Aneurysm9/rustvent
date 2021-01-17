pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut res = 0;
        for l in self.input.lines() {
            let i: u64 = l.trim().parse().expect("Unable to parse input");
            res += fuel(i);
        }
        res.to_string()
    }

    fn run_b(&self) -> String {
        let mut res = 0;
        for l in self.input.lines() {
            let i: u64 = l.trim().parse().expect("Unable to parse input");
            let mut last = fuel(i);
            while last > 0 {
                res += last;
                last = fuel(last);
            }
        }
        res.to_string()
    }
}

fn fuel(mass: u64) -> u64 {
    if mass < 6 {
        return 0;
    }
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "1"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("3405637"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("5105597"));
    }

    #[test]
    fn mass_calc() {
        assert_eq!(
            Runner {
                input: "12".to_string()
            }
            .run_a(),
            "2".to_string()
        );
        assert_eq!(
            Runner {
                input: "14".to_string()
            }
            .run_a(),
            "2".to_string()
        );
        assert_eq!(
            Runner {
                input: "1969".to_string()
            }
            .run_a(),
            "654".to_string()
        );
        assert_eq!(
            Runner {
                input: "100756".to_string()
            }
            .run_a(),
            "33583".to_string()
        );
    }

    #[test]
    fn rocket_eq() {
        assert_eq!(
            Runner {
                input: String::from("14")
            }
            .run_b(),
            String::from("2")
        );
        assert_eq!(
            Runner {
                input: String::from("1969")
            }
            .run_b(),
            String::from("966")
        );
        assert_eq!(
            Runner {
                input: String::from("100756")
            }
            .run_b(),
            String::from("50346")
        );
    }
}
