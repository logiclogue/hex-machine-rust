fn main() {
    println!("Hello, world!");
}

struct Memory(Vec<i32>);

impl Memory {
    fn read(&self, address: i32) -> i32 {
        address
    }

    //fn write(&self, address: i32, value: i32) -> Memory {

    //}
}

#[test]
fn memory_read_given_42_returns_42() {
    let memory = Memory(vec![1]);
    let address = 42;

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

    let result: i32 = my_array.iter().fold();

    assert_eq!(result, 45);
}
