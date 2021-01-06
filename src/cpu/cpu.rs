use std::rc::Rc;
use std::cell::RefCell;
use crate::cpu::registers::{Registers, RegisterType, FlagsRegister};
use crate::cpu::instructions::{Instruction, InstructionType, AddressingMode};
use crate::memory::Memory;

pub struct CPU {
    registers: Registers,
    flags_register: Rc<RefCell<FlagsRegister>>,
    memory: Rc<RefCell<Memory>>
}

impl CPU {
    pub fn new(memory: Rc<RefCell<Memory>>) -> CPU {
        let registers = Registers::new();
        let flags_register: Rc<RefCell<FlagsRegister>> = Rc::new(RefCell::new(FlagsRegister::new()));
        CPU {
            registers,
            flags_register,
            memory,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        let (address_value, address) = self.resolve_addressing_mode(instruction.get_address());
        match instruction.get_value() {
            // Load/Store Operations
            InstructionType::LDA => {
                self.load(RegisterType::A, address_value);
                self.pc_inc();
            }
            InstructionType::LDX => {
                self.load(RegisterType::X, address_value);
                self.pc_inc();
            }
            InstructionType::LDY => {
                self.load(RegisterType::Y, address_value);
                self.pc_inc();
            }
            InstructionType::STA => {
                self.store(RegisterType::A, address);
                self.pc_inc();
            }
            InstructionType::STX => {
                self.store(RegisterType::X, address);
                self.pc_inc();
            }
            InstructionType::STY => {
                self.store(RegisterType::Y, address);
                self.pc_inc();
            }

            // Register Transfers
            InstructionType::TAX => {
                self.transfer(RegisterType::A, RegisterType::X);
                self.pc_inc();
            }
            InstructionType::TAY => {
                self.transfer(RegisterType::A, RegisterType::Y);
                self.pc_inc();
            }
            InstructionType::TXA => {
                self.transfer(RegisterType::X, RegisterType::A);
                self.pc_inc();
            }
            InstructionType::TYA => {
                self.transfer(RegisterType::Y, RegisterType::A);
                self.pc_inc();
            }

            // Stack Operations
            InstructionType::TSX => {
                let stack_pointer = self.registers.get_register(RegisterType::S);
                self.registers.set_register(RegisterType::X, stack_pointer);
                (*self.flags_register.borrow_mut()).set_zero(stack_pointer == 0);
                (*self.flags_register.borrow_mut()).set_negative(((stack_pointer >> 7) & 0b1) == 1);
                self.pc_inc();
            }
            InstructionType::TXS => {
                let value = self.registers.get_register(RegisterType::X);
                self.registers.set_register(RegisterType::S, value);
                self.pc_inc();
            }
            InstructionType::PHA => {
                let value = self.registers.get_register(RegisterType::A);
                self.push(value);
                self.pc_inc();
            }
            InstructionType::PHP => {
                let status_flags: u8 = *self.flags_register.borrow_mut();
                self.push(status_flags);
                self.pc_inc();
            }
            InstructionType::PLA => {
                let value = self.pop();
                self.registers.set_register(RegisterType::A, value);
                (*self.flags_register.borrow_mut()).set_zero(value == 0);
                (*self.flags_register.borrow_mut()).set_negative(((value >> 7) & 0b1) == 1);
                self.pc_inc();
            }
            InstructionType::PLP => {
                let value = self.pop();
                (*self.flags_register.borrow_mut()).load(value);
                self.pc_inc();
            }

            // Logical
            InstructionType::AND => {
                let and_result = self.logical_and(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.registers.set_register(RegisterType::A, and_result);
                self.pc_inc();
            }
            InstructionType::EOR => {
                let xor_result = self.logical_xor(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.registers.set_register(RegisterType::A, xor_result);
                self.pc_inc();
            }
            InstructionType::ORA => {
                let or_result = self.logical_or(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.registers.set_register(RegisterType::A, or_result);
                self.pc_inc();
            }
            InstructionType::BIT => {
                self.logical_bit_test(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.pc_inc();
            }

            // Arithmetic
            InstructionType::ADC => {
                let add_result = self.arithmetic_add(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.registers.set_register(RegisterType::A, add_result);
                self.pc_inc();
            }
            InstructionType::SBC => {
                let sub_result = self.arithmetic_sub(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.registers.set_register(RegisterType::A, sub_result);
                self.pc_inc();
            }
            InstructionType::CMP => {
                self.arithmetic_cmp(
                    self.registers.get_register(RegisterType::A),
                    address_value
                );
                self.pc_inc();
            }
            InstructionType::CPX => {
                self.arithmetic_cmp(
                    self.registers.get_register(RegisterType::X),
                    address_value
                );
                self.pc_inc();
            }
            InstructionType::CPY => {
                self.arithmetic_cmp(
                    self.registers.get_register(RegisterType::Y),
                    address_value
                );
                self.pc_inc();
            }

            // Increments & Decrements
            InstructionType::INC => {
                let increment_result = self.increment(address_value);
                (*self.memory.borrow_mut()).set_byte(address, increment_result);
                self.pc_inc();
            }
            InstructionType::INX => {
                let increment_result = self.increment(self.registers.get_register(RegisterType::X));
                self.registers.set_register(RegisterType::X, increment_result);
                self.pc_inc();
            }
            InstructionType::INY => {
                let increment_result = self.increment(self.registers.get_register(RegisterType::Y));
                self.registers.set_register(RegisterType::Y, increment_result);
                self.pc_inc();
            }
            InstructionType::DEC => {
                let decrement_result = self.decrement(address_value);
                (*self.memory.borrow_mut()).set_byte(address, decrement_result);
                self.pc_inc();
            }
            InstructionType::DEX => {
                let decrement_result = self.decrement(self.registers.get_register(RegisterType::X));
                self.registers.set_register(RegisterType::X, decrement_result);
                self.pc_inc();
            }
            InstructionType::DEY => {
                let decrement_result = self.decrement(self.registers.get_register(RegisterType::Y));
                self.registers.set_register(RegisterType::Y, decrement_result);
                self.pc_inc();
            }
            
            // Shifts
            InstructionType::ASL => {
                // acc or mem
                self.pc_inc();
            }
            InstructionType::LSR => {
                // acc or mem
                self.pc_inc();
            }
            InstructionType::ROL => {
                // acc or mem
                self.pc_inc();
            }
            InstructionType::ROR => {
                // acc or mem
                self.pc_inc();
            }

            // Jumps & Calls
            InstructionType::JMP => {
                self.registers.set_pc(address);
            }
            InstructionType::JSR => {
                let pc = self.registers.get_pc();
                let msb = ((pc & 0xff00) >> 8) as u8;
                let lsb = (pc & 0x00ff) as u8;
                self.push(msb);
                self.push(lsb);
                self.registers.set_pc(address);
            }
            InstructionType::RTS => {
                let lsb = self.pop() as u16;
                let msb = self.pop() as u16;
                let address = msb * 256 + lsb;
                self.registers.set_pc(address);
            }

            // Branch
            InstructionType::BCC => {
                self.branch(address, !(*self.flags_register.borrow_mut()).get_carry());
            }
            InstructionType::BCS => {
                self.branch(address, (*self.flags_register.borrow_mut()).get_carry());
            }
            InstructionType::BEQ => {
                self.branch(address, (*self.flags_register.borrow_mut()).get_zero());
            }
            InstructionType::BMI => {
                self.branch(address, (*self.flags_register.borrow_mut()).get_negative());
            }
            InstructionType::BNE => {
                self.branch(address, !(*self.flags_register.borrow_mut()).get_zero());
            }
            InstructionType::BPL => {
                self.branch(address, !(*self.flags_register.borrow_mut()).get_negative());
            }
            InstructionType::BVC => {
                self.branch(address, !(*self.flags_register.borrow_mut()).get_overflow());
            }
            InstructionType::BVS => {
                self.branch(address, (*self.flags_register.borrow_mut()).get_overflow());
            }

            // Status Flag Changes
            InstructionType::CLC => {
                (*self.flags_register.borrow_mut()).set_carry(false);
            }
            InstructionType::CLD => {
                (*self.flags_register.borrow_mut()).set_decimal(false);
            }
            InstructionType::CLI => {
                (*self.flags_register.borrow_mut()).set_interrupt(false);
            }
            InstructionType::CLV => {
                (*self.flags_register.borrow_mut()).set_overflow(false);
            }
            InstructionType::SEC => {
                (*self.flags_register.borrow_mut()).set_carry(true);
            }
            InstructionType::SED => {
                (*self.flags_register.borrow_mut()).set_decimal(true);
            }
            InstructionType::SEI => {
                (*self.flags_register.borrow_mut()).set_interrupt(true);
            }

            // System Functions
            InstructionType::BRK => {

            }
            InstructionType::NOP => {
                self.pc_inc();
            }
            InstructionType::RTI => {

            }

            _ => panic!(),
        }
    }

    // Load/Store Operations
    fn load(&mut self, register: RegisterType, value: u8) {
        self.registers.set_register(register, value);
        (*self.flags_register.borrow_mut()).set_zero(value == 0);
        (*self.flags_register.borrow_mut()).set_negative(((value >> 7) & 0b1) == 1);
    }

    fn store(&mut self, register: RegisterType, address: u16) {
        let register_value = self.registers.get_register(register);
        (*self.memory.borrow_mut()).set_byte(address, register_value);
    }

    // Register Transfers
    fn transfer(&mut self, from: RegisterType, to: RegisterType) {
        let register_value = self.registers.get_register(from);
        self.registers.set_register(to, register_value);
        (*self.flags_register.borrow_mut()).set_zero(register_value == 0);
        (*self.flags_register.borrow_mut()).set_negative(((register_value >> 7) & 0b1) == 1);
    }

    // Stack Operations
    fn push(&mut self, value: u8) {
        let address = (self.registers.get_register(RegisterType::S) as u16) + 0x0100;
        (*self.memory.borrow_mut()).set_byte(address, value);
        self.registers.push_stack();
    }

    fn pop(&mut self) -> u8 {
        let address = (self.registers.get_register(RegisterType::S) as u16) + 0x0100;
        let value = (*self.memory.borrow_mut()).get_byte(address);
        self.registers.pop_stack();
        value
    }

    // Logical Functions
    pub fn logical_and(&mut self, reg_value: u8, mem_value: u8) -> u8 {
        let and_result = reg_value & mem_value;
        (*self.flags_register.borrow_mut()).set_zero(and_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((and_result >> 7) & 0b1) == 1);
        and_result
    }

    pub fn logical_xor(&mut self, reg_value: u8, mem_value: u8) -> u8 {
        let xor_result = reg_value ^ mem_value;
        (*self.flags_register.borrow_mut()).set_zero(xor_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((xor_result >> 7) & 0b1) == 1);
        xor_result
    }

    pub fn logical_or(&mut self, reg_value: u8, mem_value: u8) -> u8 {
        let or_result = reg_value | mem_value;
        (*self.flags_register.borrow_mut()).set_zero(or_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((or_result >> 7) & 0b1) == 1);
        or_result
    }

    pub fn logical_bit_test(&mut self, reg_value: u8, mem_value: u8) {
        let and_result = reg_value & mem_value;
        (*self.flags_register.borrow_mut()).set_zero(and_result == 0);
        (*self.flags_register.borrow_mut()).set_overflow(((mem_value >> 6) & 0b1) == 1);
        (*self.flags_register.borrow_mut()).set_negative(((mem_value >> 7) & 0b1) == 1);
    }

    // Arithmetic Functions
    pub fn arithmetic_add(&mut self, reg_value: u8, mem_value: u8) -> u8 {
        let carry = if (*self.flags_register.borrow_mut()).get_carry() { 1 } else { 0 };
        let signed_reg_value: i16 = (reg_value as i8) as i16;
        let signed_mem_value: i16 = (mem_value as i8) as i16;
        let sum: i16 = signed_reg_value + signed_mem_value + carry;
        let unsigned_sum: u8 = (sum & 0xff) as u8;
        (*self.flags_register.borrow_mut()).set_carry(((sum >> 8) & 0b1) == 1);
        (*self.flags_register.borrow_mut()).set_zero(unsigned_sum == 0);
        (*self.flags_register.borrow_mut()).set_overflow(((sum >> 8) & 0b1) != ((sum >> 7) & 0b1));
        (*self.flags_register.borrow_mut()).set_negative(((sum >> 7) & 0b1) == 1);
        unsigned_sum
    }

    pub fn arithmetic_sub(&mut self, reg_value: u8, mem_value: u8) -> u8 {
        let carry = if (*self.flags_register.borrow_mut()).get_carry() { 1 } else { 0 };
        let signed_reg_value: i16 = (reg_value as i8) as i16;
        let signed_mem_value: i16 = (mem_value as i8) as i16;
        let sub: i16 = signed_reg_value - signed_mem_value - (1 - carry);
        let unsigned_sub: u8 = (sub & 0xff) as u8;
        (*self.flags_register.borrow_mut()).set_carry(((sub >> 8) & 0b1) == 0);
        (*self.flags_register.borrow_mut()).set_zero(unsigned_sub == 0);
        (*self.flags_register.borrow_mut()).set_overflow(((sub >> 8) & 0b1) != ((sub >> 7) & 0b1));
        (*self.flags_register.borrow_mut()).set_negative(((sub >> 7) & 0b1) == 1);
        unsigned_sub
    }

    pub fn arithmetic_cmp(&mut self, reg_value: u8, mem_value: u8) {
        let signed_reg_value: i16 = (reg_value as i8) as i16;
        let signed_mem_value: i16 = (mem_value as i8) as i16;
        let sub: i16 = signed_reg_value - signed_mem_value;
        (*self.flags_register.borrow_mut()).set_carry(signed_reg_value >= signed_mem_value);
        (*self.flags_register.borrow_mut()).set_zero(signed_reg_value == signed_mem_value);
        (*self.flags_register.borrow_mut()).set_negative(((sub >> 7) & 0b1) == 1);
    }

    // Increments & Decrements
    pub fn increment(&mut self, value: u8) -> u8 {
        let incremented = (((value as u16) + 1) & 0xff) as u8;
        (*self.flags_register.borrow_mut()).set_zero(incremented == 0);
        (*self.flags_register.borrow_mut()).set_negative(((incremented >> 7) & 0b1) == 1);
        incremented
    }

    pub fn decrement(&mut self, value: u8) -> u8 {
        let decremented = (((value as u16) - 1) & 0xff) as u8;
        (*self.flags_register.borrow_mut()).set_zero(decremented == 0);
        (*self.flags_register.borrow_mut()).set_negative(((decremented >> 7) & 0b1) == 1);
        decremented
    }

    // Shifts
    pub fn arithmetic_shift_left(&mut self, value: u8) -> u8 {
        let shift_result = (((value as u16) * 2) & 0xff) as u8;
        (*self.flags_register.borrow_mut()).set_carry(((value >> 7) & 0b1) == 1);
        // TODO: zero only if in accumulator mode
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 7) & 0b1) == 1);
        shift_result
    }

    pub fn logical_shift_right(&mut self, value: u8) -> u8 {
        let shift_result = value >> 1;
        (*self.flags_register.borrow_mut()).set_carry(((value >> 7) & 0b1) == 1);
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 7) & 0b1) == 1);
        shift_result
    }

    pub fn rotate_left(&mut self, value: u8) -> u8 {
        let shift_result = ((((value as u16) * 2) & 0xff) as u8) | (value & 0x80);
        (*self.flags_register.borrow_mut()).set_carry(((value >> 7) & 0b1) == 1);
        // TODO: zero only if in accumulator mode
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 7) & 0b1) == 1);
        shift_result
    }

    pub fn rotate_left(&mut self, value: u8) -> u8 {
        let shift_result = (value >> 1) | (value & 0x01);
        (*self.flags_register.borrow_mut()).set_carry(((value >> 7) & 0b1) == 1);
        // TODO: zero only if in accumulator mode
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 7) & 0b1) == 1);
        shift_result
    }

    // Branch
    pub fn pc_inc(&mut self) {
        self.registers.change_pc(1);
    }

    pub fn branch(&mut self, address: u16, flag: bool) {
        if (flag) {
            self.registers.change_pc(address);
        }
    }

    // Addressing
    // TODO: implement all addressing modes (with acc in rotates in mind)
    fn resolve_addressing_mode(&self, addressing_mode: AddressingMode) -> (u8, u16) {
        match addressing_mode {
            AddressingMode::Relative(relative) => {
                let signed: bool = ((relative >> 7) & 0b1) == 1;
                let value: u8 = relative & 0x7f;
                let signed_value = if signed { !value + 1 } else { value };
                (signed_value, signed_value as u16)
            }
            AddressingMode::Immediate(immediate) => {
                (immediate, immediate as u16)
            }
            AddressingMode::ZeroPage(address, register) => {
                // ZeroPage
                let mut register_value: u8 = 0;
                // ZeroPageX
                if register == 1 {
                    register_value = self.registers.get_register(RegisterType::X);
                }
                // ZeroPageY
                if register == 2 {
                    register_value = self.registers.get_register(RegisterType::Y);
                }
                let new_address: u8 = address + register_value;
                let address_value: u8 = (*self.memory.borrow_mut()).get_byte(new_address as u16);
                (address_value as u8, new_address as u16)
            }
            AddressingMode::Absolute(address, register) => {
                // Absolute
                let mut register_value: u16 = 0;
                // AbsoluteX
                if register == 1 {
                    register_value = self.registers.get_register(RegisterType::X) as u16;
                }
                // AbsoluteY
                if register == 2 {
                    register_value = self.registers.get_register(RegisterType::Y) as u16;
                }
                let new_address: u16 = address + register_value;
                let address_value: u8 = (*self.memory.borrow_mut()).get_byte(new_address);
                (address_value as u8, new_address as u16)
            }
            _ => { (0, 0) }
        }
    }
}