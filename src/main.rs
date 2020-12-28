#[allow(dead_code)]
mod cpu;
#[allow(dead_code)]
mod memory;

use std::rc::Rc;
use std::cell::RefCell;
use cpu::{CPU, Instruction, AddressingMode};
use memory::{Memory};

fn main() {
    let mem1: Rc<RefCell<Memory>> = Rc::new(RefCell::new(Memory::new()));
    let mut cpu1: CPU = CPU::new(Rc::clone(&mem1));
    (*mem1.borrow_mut()).set_byte(120, 10);
    let instruction1: Instruction = Instruction::LDA(AddressingMode::ZeroPage(120, 0));
    let instruction2: Instruction = Instruction::ADC(AddressingMode::Immediate(4));
    cpu1.execute(instruction1);
    println!("{}", cpu1.execute(instruction2));
}
