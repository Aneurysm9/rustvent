use super::intcode;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut vm = intcode::Vm::new(&self.input);
        vm.input(1);
        assert_eq!(vm.run().is_ok(), true);
        while let Some(o) = vm.pop_output() {
            if o != 0 {
                return o.to_string();
            }
        }
        String::from("Error!")
    }

    fn run_b(&self) -> String {
        let mut vm = intcode::Vm::new(&self.input);
        vm.input(5);
        assert_eq!(vm.run().is_ok(), true);
        vm.pop_output().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "5"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("13818007"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("3176266"));
    }
}
