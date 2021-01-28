use super::intcode;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        exec(&self.input, 12, 2).to_string()
    }

    fn run_b(&self) -> String {
        for noun in 0..=99 {
            for verb in 0..=99 {
                if 19_690_720 == exec(&self.input, noun, verb) {
                    return (100 * noun + verb).to_string();
                }
            }
        }
        String::from("Error!")
    }
}

fn exec(input: &str, noun: i64, verb: i64) -> i64 {
    let mut vm = intcode::Vm::new(input);
    assert_eq!(vm.set(1, noun).is_ok(), true);
    assert_eq!(vm.set(2, verb).is_ok(), true);
    assert_eq!(vm.run().is_ok(), true);
    vm.get(0, Some(&intcode::ParameterMode::Immediate))
        .unwrap()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2019, "2"),
        }
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("5434663"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("4559"));
    }
}
