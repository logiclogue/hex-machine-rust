fn main() {
    println!("Hello, world!");
}

struct Memory(Vec<u8>);

impl Memory {
    fn read(&self, address: u32) -> u8 {
        42
    }

    fn write(self, address: u32, value: u8) -> Memory {
        self
    }
}

#[test]
fn memory_read_given_42_returns_42() {
    let address = 145;
    let value = 42;
    let memory = Memory(vec![1]).write(address, value);

    assert_eq!(memory.read(address), 42);
}

#[test]
fn memory_read_given_1_returns_1() {
    let memory = Memory(vec![1]);
    let address = 1;

    assert_eq!(memory.read(address), 1);
}

#[test]
fn sandbox() {
    let my_array = vec![42, 1, 2];

    let result: u8 = my_array.iter().sum();

    assert_eq!(result, 45);
}
