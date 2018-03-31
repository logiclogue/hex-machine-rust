use machine::Machine;

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
    pub fn execute(self, machine: Machine) -> Machine {
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