mod machine;
mod memory;
mod instruction;

use machine::Machine;
use memory::Memory;
use instruction::Instruction;

fn main() {
    let mut memory = Memory::new();

    memory.write(0x00, 42);

    println!("{}", memory.read(0x00));
}
