pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        self.run(2020)
    }

    fn run_b(&self) -> String {
        self.run(30000000)
    }
}

impl Runner {
    fn run(&self, at: usize) -> String {
        let start = self.input.matches(',').count() + 2;
        Memory::new(
            at,
            self.input.trim().split(',').map(|c| {
                c.parse::<usize>()
                    .unwrap_or_else(|_| panic!("Unable to parse input value {}", c))
            }),
        )
        .nth(at - start)
        .unwrap()
        .to_string()
    }
}

#[derive(Debug)]
struct Memory {
    history: Vec<usize>,
    round: usize,
    last: usize,
}

impl Memory {
    fn new(size: usize, start: impl Iterator<Item = usize>) -> Memory {
        let mut mem = vec![0; size];
        let mut round = 0;
        let mut last = 0;
        for v in start {
            round += 1;
            mem[v] = round;
            last = v;
        }
        Memory {
            history: mem,
            round,
            last,
        }
    }
}

impl Iterator for Memory {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = 0;
        if let Some(last) = self.history.get(self.last) {
            if *last != 0 {
                res = self.round - *last;
            }
        }
        self.history[self.last] = self.round;
        self.last = res;
        self.round += 1;

        Some(res)
    }
}
