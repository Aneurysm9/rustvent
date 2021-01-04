use std::{collections::HashSet, panic};

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut vm = Vm::new(&self.input);
        vm.run();
        vm.acc.to_string()
    }

    fn run_b(&self) -> String {
        let vm = Vm::new(&self.input);
        for (idx, instr) in vm.mem.clone().into_iter().enumerate() {
            let mut vm2 = vm.clone();
            match instr.op {
                Opcode::Jmp => vm2.mem[idx].op = Opcode::Nop,
                Opcode::Nop => vm2.mem[idx].op = Opcode::Jmp,
                _ => (),
            }
            if vm2.run() {
                return vm2.acc.to_string();
            }
        }
        String::from("")
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Opcode,
    arg: isize,
}

#[derive(Debug, Clone)]
struct Vm {
    mem: Vec<Instruction>,
    acc: isize,
    pc: usize,
    seen: HashSet<usize>,
}

impl Vm {
    pub fn new(input: &str) -> Vm {
        Vm {
            mem: input
                .trim()
                .lines()
                .map(|l| {
                    let instr: Vec<&str> = l.split_ascii_whitespace().collect();
                    Instruction {
                        op: match instr[0] {
                            "nop" => Opcode::Nop,
                            "acc" => Opcode::Acc,
                            "jmp" => Opcode::Jmp,
                            _ => panic!("Invalid opcode encountered during parsing"),
                        },
                        arg: instr[1].parse().unwrap_or_else(|_| {
                            panic!("Unable to parse argument \"{}\"", instr[1])
                        }),
                    }
                })
                .collect(),
            acc: 0,
            pc: 0,
            seen: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> bool {
        while self.pc < self.mem.len() {
            if self.seen.contains(&self.pc) {
                return false;
            }
            self.seen.insert(self.pc);
            let op = &self.mem[self.pc];
            match op.op {
                Opcode::Acc => self.acc += op.arg,
                Opcode::Jmp => self.pc = (self.pc as isize + op.arg) as usize,
                Opcode::Nop => (),
            }
            if op.op != Opcode::Jmp {
                self.pc += 1;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{read_input, Solution};

    fn new() -> Runner {
        Runner {
            input: read_input(2020, "8"),
        }
    }

    fn simple() -> Runner {
        Runner {
            input: read_input(2020, "8_simple"),
        }
    }

    #[test]
    fn simple_a() {
        assert_eq!(simple().run_a(), String::from("5"));
    }

    #[test]
    fn simple_b() {
        assert_eq!(simple().run_b(), String::from("8"));
    }

    #[test]
    fn real_a() {
        assert_eq!(new().run_a(), String::from("1818"));
    }

    #[test]
    fn real_b() {
        assert_eq!(new().run_b(), String::from("631"));
    }
}
