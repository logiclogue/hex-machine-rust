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
    pub fn execute(self, machine: Machine, mut memory: Memory) -> (Machine, Memory) {
        let read_value = memory.read(machine.o_reg);

        (match self {
            Instruction::LDAM => machine.set_a_reg(read_value).set_o_reg(0),
            Instruction::LDBM => machine.set_b_reg(read_value).set_o_reg(0),
            Instruction::STAM => {
                memory.write(machine.o_reg, machine.a_reg);

                machine.set_o_reg(0)
            },
            _                 => machine
        }, memory)
    }
}

#[test]
fn execute_ldam_sets_o_reg_to_0() {
    let instruction = Instruction::LDAM;
    let mut machine = Machine::new();
    let mut memory = Memory::new();

    let result = instruction.execute(machine, memory);

    assert_eq!(result.0.o_reg, 0);
}


#[test]
fn execute_ldam_sets_a_reg_to_memory_value() {
    let instruction = Instruction::LDAM;
    let mut machine = Machine::new();
    let mut memory = Memory::new();
    let address = 10;

    memory.write(address, 42);

    machine.o_reg = address;

    let result = instruction.execute(machine, memory);

    assert_eq!(result.0.a_reg, 42);
}

#[test]
fn execute_ldbm_sets_o_reg_to_0() {
    let instruction = Instruction::LDBM;
    let machine = Machine::new();
    let memory = Memory::new();

    let result = instruction.execute(machine, memory);

    assert_eq!(result.0.o_reg, 0);
}

#[test]
fn execute_ldbm_sets_b_reg_to_memory_value() {
    let instruction = Instruction::LDBM;
    let mut memory = Memory::new();
    let address = 10;
    let machine = Machine::new().set_o_reg(address);

    memory.write(address, 42);

    let result = instruction.execute(machine, memory);

    assert_eq!(result.0.b_reg, 42);
}

#[test]
fn execute_stam_sets_o_reg_to_0() {
    let instruction = Instruction::STAM;
    let mut memory = Memory::new();
    let address = 10;
    let machine = Machine::new().set_o_reg(address);

    let result = instruction.execute(machine, memory);

    assert_eq!(result.0.o_reg, 0);
}

#[test]
fn execute_stam_stores_a_reg_in_memory() {
    let instruction = Instruction::STAM;
    let mut memory = Memory::new();
    let address = 10;
    let value = 42;
    let machine = Machine::new()
        .set_o_reg(address)
        .set_a_reg(value);

    let result = instruction.execute(machine, memory);

    memory = result.1;

    assert_eq!(memory.read(address), value);
}
