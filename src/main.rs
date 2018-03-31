mod machine;
mod memory;
mod instruction;

use memory::Memory;

fn main() {
    let mut memory = Memory::new();

    memory.write(0x00, 42);

    println!("{}", memory.read(0x00));
}
