pub struct Memory([u8; 256]);

impl Memory {
    pub fn new() -> Self {
        Memory([0; 256])
    }

    pub fn read(&self, address: u8) -> u8 {
        self.0[address as usize]
    }

    pub fn write(&mut self, address: u8, value: u8) -> &Memory {
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
