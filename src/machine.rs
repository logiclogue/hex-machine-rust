pub struct Machine {
    pub o_reg: u8,
    pub a_reg: u8,
    pub b_reg: u8,
    pub pc: u8,
    pub running: bool
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            o_reg: 0,
            a_reg: 0,
            b_reg: 0,
            pc: 0,
            running: false
        }
    }

    pub fn set_o_reg(self, value: u8) -> Self {
        Machine {
            o_reg: value,
            a_reg: self.a_reg,
            b_reg: self.b_reg,
            pc: self.pc,
            running: self.running
        }
    }

    pub fn set_a_reg(self, value: u8) -> Self {
        Machine {
            o_reg: self.o_reg,
            a_reg: value,
            b_reg: self.b_reg,
            pc: self.pc,
            running: self.running
        }
    }
}

#[test]
fn test_machine_constructor_o_reg_0() {
    let machine = Machine::new();

    assert_eq!(machine.o_reg, 0);
}

#[test]
fn test_machine_constructor_a_reg_0() {
    let machine = Machine::new();

    assert_eq!(machine.a_reg, 0);
}

#[test]
fn test_machine_constructor_b_reg_0() {
    let machine = Machine::new();

    assert_eq!(machine.b_reg, 0);
}

#[test]
fn test_machine_constructor_pc_0() {
    let machine = Machine::new();

    assert_eq!(machine.pc, 0);
}

#[test]
fn test_machine_constructor_running_false() {
    let machine = Machine::new();

    assert_eq!(machine.running, false);
}
