pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let card = self
            .input
            .trim()
            .lines()
            .next()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let door = self
            .input
            .trim()
            .lines()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let subj: usize = 7;
        let sz: usize = 20_201_227;

        let mut cur = 1;
        let mut rounds = 0;
        while cur != card {
            cur = (cur * subj) % sz;
            rounds += 1;
        }
        let mut key = 1;
        for _ in 0..rounds {
            key = (key * door) % sz;
        }
        key.to_string()
    }

    fn run_b(&self) -> String {
        String::from("Happy AoC!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "25"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "25_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("14897079"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("448851"));
    }
}
