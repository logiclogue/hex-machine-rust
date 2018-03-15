fn main() {
    let mut memory = Memory::new();

    memory.write(0x00, 42);

    println!("{}", memory.read(0x00));
}

struct Machine {
    o_reg: u8,
    a_reg: u8,
    b_reg: u8,
    pc: u8,
    running: bool
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

#[test]
fn test_instruction() {
    let instruction = Instruction::LDAM;

    match instruction {
        Instruction::LDAM => assert!(true),
        _                 => assert!(false)
    }
}
