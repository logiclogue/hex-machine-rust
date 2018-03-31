mod machine;

use machine::Machine;

fn main() {
    let mut memory = Memory::new();

    memory.write(0x00, 42);

    println!("{}", memory.read(0x00));
}

struct Memory([u8; 256]);

impl Memory {
    fn new() -> Self {
        Memory([0; 256])
    }

    fn read(&self, address: u8) -> u8 {
        self.0[address as usize]
    }

    fn write(&mut self, address: u8, value: u8) -> &Memory {
        self.0[address as usize] = value;

        self
    }
}

#[test]
fn test_memory_write_at_42_100_reads_100() {
    let mut memory = Memory::new();

    memory.write(42, 100);

    assert_eq!(memory.read(42), 100);
}

enum Instruction {
    LDAM,
    LDBM,
    STAM,
    LDAC,
    LDBC,
    LDAP,
    LDAI,
    LDBI,
    STAI,
    BR,
    BRZ,
    BRN,
    BRB,
    ADD,
    SUB,
    PFIX
}

impl Instruction {
    fn execute(self, machine: Machine) -> Machine {
        machine
    }
}

#[test]
fn test_instruction() {
    let instruction = Instruction::LDAM;

    match instruction {
        Instruction::LDAM => assert!(true),
        _                 => assert!(false)
    }
}

#[test]
fn instruction_execute_ldam_works() {
    let instruction = Instruction::LDAM;
    let mut machine = Machine::new();

    machine.o_reg = 10;

    machine = instruction.execute(machine);

    assert_eq!(machine.o_reg, 0);
}
