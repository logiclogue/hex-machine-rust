use machine::Machine;
use memory::Memory;

pub enum Instruction {
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
    pub fn execute(self, machine: Machine, memory: Memory) -> Machine {
        let read_value = memory.read(machine.o_reg);

        machine.set_o_reg(0).set_a_reg(read_value)
    }
}

#[test]
fn execute_ldam_sets_o_reg_to_0() {
    let instruction = Instruction::LDAM;
    let mut machine = Machine::new();
    let mut memory = Memory::new();

    machine = instruction.execute(machine, memory);

    assert_eq!(machine.o_reg, 0);
}


#[test]
fn execute_ldam_sets_a_reg_to_memory_value() {
    let instruction = Instruction::LDAM;
    let mut machine = Machine::new();
    let mut memory = Memory::new();
    let address = 10;

    memory.write(address, 42);

    machine.o_reg = address;

    machine = instruction.execute(machine, memory);

    assert_eq!(machine.a_reg, 42);
}
