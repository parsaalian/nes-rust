#[allow(dead_code)]
mod cpu;
use cpu::{CPU, Instruction, AddressingMode};

fn main() {
    let mut cpu1: CPU = CPU::new();
    let instruction1: Instruction = Instruction::LDA(AddressingMode::Immediate(250));
    let instruction2: Instruction = Instruction::ADC(AddressingMode::Immediate(4));
    cpu1.execute(instruction1);
    println!("{}", cpu1.execute(instruction2));
}
