pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let card = self
            .input
            .trim()
            .lines()
            .nth(0)
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
        let sz: usize = 20201227;

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
