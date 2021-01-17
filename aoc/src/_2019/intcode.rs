use std::fmt;

#[derive(Debug, Clone)]
pub struct Vm {
    memory: Memory,
    pc: usize,
}

#[derive(Debug, Clone)]
struct Memory {
    mem: Vec<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add(i64, i64, i64),
    Mul(i64, i64, i64),
    Halt,
}

impl Opcode {
    fn len(&self) -> usize {
        match self {
            Opcode::Add(_, _, _) => 4,
            Opcode::Mul(_, _, _) => 4,
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
        }
    }

    pub fn run(&mut self) -> Result<(), VmRuntimeError> {
        while self.pc < self.memory.len() {
            match self.next_opcode() {
                Ok(Opcode::Halt) => return Ok(()),
                Ok(op) => {
                    if self.execute(op).is_err() {
                        return Err(VmRuntimeError(self.pc));
                    }
                    self.pc += op.len();
                }
                Err(e) => return Err(e),
            };
        }
        Ok(())
    }

    fn execute(&mut self, op: Opcode) -> Result<(), MemSetError> {
        match op {
            Opcode::Add(a, b, c) => self.set(c as usize, a + b),
            Opcode::Mul(a, b, c) => self.set(c as usize, a * b),
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
            Some(99) => Ok(Opcode::Halt),
            _ => Err(VmRuntimeError(self.pc)),
        }
    }

    pub fn get(&self, addr: usize) -> Option<&i64> {
        self.memory.get(addr)
    }

    pub fn set(&mut self, addr: usize, val: i64) -> Result<(), MemSetError> {
        self.memory.set(addr, val)
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

    fn set(&mut self, addr: usize, val: i64) -> Result<(), MemSetError> {
        if addr > self.mem.len() {
            return Err(MemSetError);
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
        if let Err(e) = m.set(3, 33) {
            panic!(e);
        }
        assert_eq!(m.get(3), Some(&33));
        if m.set(99, 99).is_ok() {
            panic!("Unexpected Ok writing to invalid memory address");
        }
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
