#[allow(dead_code)]
mod cpu;
#[allow(dead_code)]
mod memory;
#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;

use std::rc::Rc;
use std::cell::RefCell;
use memory::{Memory};
use cpu::cpu::{CPU};
use cpu::instructions::{InstructionReader, Instruction};

fn main() {
    /*
    let instruction1: Instruction = Instruction::LDA(AddressingMode::ZeroPage(120, 0));
    let instruction2: Instruction = Instruction::ADC(AddressingMode::Immediate(4));
    cpu1.execute(instruction1);
    println!("{}", cpu1.execute(instruction2));
    */
    let mut instruction_reader: InstructionReader = InstructionReader::new();
    let instruction1: Instruction = instruction_reader.read("A901");
    let mem1: Rc<RefCell<Memory>> = Rc::new(RefCell::new(Memory::new()));
    let mut cpu1: CPU = CPU::new(Rc::clone(&mem1));
    cpu1.execute(instruction1);
    println!("{}", ((255 as u8) as i8) as i16);
}
