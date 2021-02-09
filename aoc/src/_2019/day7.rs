use super::intcode;
use itertools::Itertools;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut best = 0;
        for perm in vec![0, 1, 2, 3, 4].iter().permutations(5) {
            let res = self.eval_a(perm);
            if res > best {
                best = res;
            }
        }
        best.to_string()
    }

    fn run_b(&self) -> String {
        let mut best = 0;
        for perm in vec![5, 6, 7, 8, 9].iter().permutations(5) {
            let res = self.eval_b(perm);
            if res > best {
                best = res;
            }
        }
        best.to_string()
    }
}

impl Runner {
    fn eval_a(&self, order: Vec<&i64>) -> i64 {
        let mut res = 0;
        for phase in order {
            let mut amp = intcode::Vm::new(&self.input);
            amp.input(*phase);
            amp.input(res);
            amp.run().expect("Error running amplifier");
            res = amp.pop_output().unwrap();
        }
        res
    }

    fn eval_b(&self, order: Vec<&i64>) -> i64 {
        let mut amps = Vec::new();
        for i in 0..5 {
            let mut amp = intcode::Vm::new(&self.input);
            amp.input(**order.get(i).unwrap());
            amps.push(amp);
        }

        let mut res = 0;
        let mut done = false;

        while !done {
            for i in 0..5 {
                let amp = amps.get_mut(i).unwrap();
                amp.input(res);
                if let Ok(()) = amp.run() {
                    done = true;
                }
                res = amp.pop_output().unwrap();
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "7"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("255840"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("84088865"));
    }
}
