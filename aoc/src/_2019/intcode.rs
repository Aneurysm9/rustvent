use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Vm {
    memory: Memory,
    pc: usize,
    input: Option<i64>,
    output: Vec<i64>,
}

#[derive(Debug, Clone)]
struct Memory {
    mem: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add(i64, i64, i64),
    Mul(i64, i64, i64),
    Input(i64),
    Output(i64),
    Halt,
}

impl Opcode {
    fn len(&self) -> usize {
        match self {
            Opcode::Add(_, _, _) => 4,
            Opcode::Mul(_, _, _) => 4,
            Opcode::Input(_) => 2,
            Opcode::Output(_) => 2,
            Opcode::Halt => 1,
        }
    }
}

impl Vm {
    pub fn new(input: &str) -> Vm {
        Vm {
            memory: Memory::new(
                input
                    .trim()
                    .split(',')
                    .map(|v| v.parse::<i64>().unwrap())
                    .collect(),
            ),
            pc: 0,
            input: None,
            output: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        while self.pc < self.memory.len() {
            match self.next_opcode() {
                Ok(Opcode::Halt) => return Ok(()),
                Ok(op) => {
                    if let Err(e) = self.execute(op) {
                        return Err(e);
                    }
                    self.pc += op.len();
                }
                Err(e) => return Err(e.into()),
            };
        }
        Ok(())
    }

    fn execute(&mut self, op: Opcode) -> Result<(), Box<dyn Error>> {
        match op {
            Opcode::Add(a, b, c) => self.set(c as usize, a + b),
            Opcode::Mul(a, b, c) => self.set(c as usize, a * b),
            Opcode::Input(a) => {
                if let Some(val) = self.input {
                    self.input = None;
                    self.set(a as usize, val)
                } else {
                    Err(InputRequired.into())
                }
            }
            Opcode::Output(a) => {
                self.output.push(*self.get(a as usize).unwrap());
                Ok(())
            }
            Opcode::Halt => Ok(()),
        }
    }

    fn next_opcode(&self) -> Result<Opcode, VmRuntimeError> {
        match self.get(self.pc) {
            Some(1) => Ok(Opcode::Add(
                *self.memory.get_pos(self.pc + 1).unwrap(),
                *self.memory.get_pos(self.pc + 2).unwrap(),
                *self.get(self.pc + 3).unwrap(),
            )),
            Some(2) => Ok(Opcode::Mul(
                *self.memory.get_pos(self.pc + 1).unwrap(),
                *self.memory.get_pos(self.pc + 2).unwrap(),
                *self.get(self.pc + 3).unwrap(),
            )),
            Some(3) => Ok(Opcode::Input(*self.memory.get(self.pc + 1).unwrap())),
            Some(4) => Ok(Opcode::Output(*self.memory.get(self.pc + 1).unwrap())),
            Some(99) => Ok(Opcode::Halt),
            _ => Err(VmRuntimeError(self.pc)),
        }
    }

    pub fn get(&self, addr: usize) -> Option<&i64> {
        self.memory.get(addr)
    }

    pub fn set(&mut self, addr: usize, val: i64) -> Result<(), Box<dyn Error>> {
        self.memory.set(addr, val)
    }

    pub fn input(&mut self, input: i64) {
        self.input = Some(input)
    }
}

#[derive(Debug, Clone)]
pub struct VmRuntimeError(usize);

impl fmt::Display for VmRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "an error was encountered executing instruction at {}",
            self.0
        )
    }
}

impl Error for VmRuntimeError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputRequired;

impl fmt::Display for InputRequired {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Input is required")
    }
}

impl Error for InputRequired {}

impl Memory {
    fn new(mem: Vec<i64>) -> Memory {
        Memory { mem }
    }

    fn get_pos(&self, addr: usize) -> Option<&i64> {
        if let Some(pos) = self.get(addr) {
            return self.get(*pos as usize);
        }
        None
    }

    fn get(&self, addr: usize) -> Option<&i64> {
        self.mem.get(addr)
    }

    fn set(&mut self, addr: usize, val: i64) -> Result<(), Box<dyn Error>> {
        if addr > self.mem.len() {
            return Err(MemSetError.into());
        }

        self.mem[addr] = val;
        Ok(())
    }

    fn len(&self) -> usize {
        self.mem.len()
    }
}

#[derive(Debug, Clone)]
pub struct MemSetError;

impl fmt::Display for MemSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "attempt to set at invalid memory address")
    }
}

impl Error for MemSetError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_read() {
        let m = Memory::new(vec![2, 10, 20, 30]);
        assert_eq!(m.get(3), Some(&30));
        assert_eq!(m.get(0), Some(&2));
        assert_eq!(m.get(99), None);
        assert_eq!(m.get_pos(0), Some(&20));
        assert_eq!(m.get_pos(2), None);
        assert_eq!(m.get_pos(99), None);
    }

    #[test]
    fn memory_write() {
        let mut m = Memory::new(vec![2, 10, 20, 30]);
        assert_eq!(m.get(3), Some(&30));
        if m.set(3, 33).is_err() {
            panic!("Failed to set memory");
        }
        assert_eq!(m.get(3), Some(&33));
        if m.set(99, 99).is_ok() {
            panic!("Unexpected Ok writing to invalid memory address");
        }
    }

    #[test]
    fn io() {
        let mut vm = Vm::new("3,0,4,0,99");
        if vm.run().err().unwrap().downcast::<InputRequired>().is_err() {
            assert_eq!(true, false);
        }
        vm.input(10);
        assert_eq!(vm.pc, 0);
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.output[0], 10);
    }

    #[test]
    fn vm_run() {
        let mut vm = Vm::new("1,9,10,3,2,3,11,0,99,30,40,50");
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.get(3), Some(&70));

        let mut vm = Vm::new("1,0,0,0,99");
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.get(0), Some(&2));

        let mut vm = Vm::new("2,3,0,3,99");
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.get(3), Some(&6));

        let mut vm = Vm::new("2,4,4,5,99,0");
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.get(5), Some(&9801));

        let mut vm = Vm::new("1,1,1,4,99,5,6,0,99");
        assert_eq!(vm.run().is_ok(), true);
        assert_eq!(vm.get(0), Some(&30));
    }
}
