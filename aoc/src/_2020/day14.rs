use std::collections::HashMap;

pub struct Runner {
    pub input: String,
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Mem(u64, u64),
}

#[derive(Debug, Clone)]
struct Mask {
    and: u64,
    or: u64,
    float: u64,
}

impl Mask {
    fn new() -> Mask {
        Mask {
            and: 0xFFFF_FFFF_FFFF_FFFF,
            or: 0,
            float: 0,
        }
    }

    fn apply(&self, val: &u64) -> u64 {
        let mut out = *val;
        out &= self.and;
        out |= self.or;
        out
    }

    fn iter(&self, addr: &u64) -> MaskIter {
        MaskIter::new(addr, &self)
    }
}

#[derive(Debug, Clone)]
struct MaskIter {
    val: u64,
    mask: Mask,
    cur: u64,
    started: bool,
}

impl MaskIter {
    fn new(val: &u64, mask: &Mask) -> MaskIter {
        let mut v = *val | mask.or;
        v &= !mask.float;
        MaskIter {
            val: v,
            mask: mask.clone(),
            cur: 0,
            started: false,
        }
    }
}

impl Iterator for MaskIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.cur = (self.cur - self.mask.float) & self.mask.float;
        }

        if self.cur == 0 && self.started {
            None
        } else if !self.started {
            self.started = true;
            Some(self.val)
        } else {
            Some(self.val | self.cur)
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.trim().lines().filter_map(|l| {
        let parts: Vec<&str> = l.split('=').collect();
        match &parts[0][0..2] {
            "ma" => {
                let mut mask = Mask::new();
                for (i, b) in parts[1].trim().chars().enumerate() {
                    let offset = 35 - i as u64;
                    match b {
                        '1' => mask.or += 1 << offset,
                        '0' => mask.and -= 1 << offset,
                        'X' => mask.float += 1 << offset,
                        v => panic!("Unexpected value found in mask: {}", v),
                    }
                }
                Some(Instruction::Mask(mask))
            }
            "me" => {
                let addr = &parts[0][4..parts[0].len() - 2];
                Some(Instruction::Mem(
                    addr.parse().unwrap_or_else(|e| {
                        panic!("Unable to parse memory address {}: {}", addr, e)
                    }),
                    parts[1].trim().parse().unwrap_or_else(|e| {
                        panic!("Unable to parse assignment value {}: {}", parts[1], e)
                    }),
                ))
            }
            i => {
                println!("Unexpected input: {}", i);
                None
            }
        }
    })
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let mut mem = HashMap::new();
        let mut mask = Mask::new();
        for instr in parse_input(&self.input) {
            match instr {
                Instruction::Mask(m) => mask = m,
                Instruction::Mem(addr, val) => {
                    mem.insert(addr, mask.apply(&val));
                }
            }
        }
        mem.values().sum::<u64>().to_string()
    }

    fn run_b(&self) -> String {
        let mut mem = HashMap::new();
        let mut mask = Mask::new();
        for instr in parse_input(&self.input) {
            match instr {
                Instruction::Mask(m) => mask = m,
                Instruction::Mem(addr, val) => {
                    for tgt in mask.iter(&addr) {
                        mem.insert(tgt, val);
                    }
                }
            }
        }
        mem.values().sum::<u64>().to_string()
    }
}
