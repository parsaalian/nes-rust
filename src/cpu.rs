use std::rc::Rc;
use std::cell::RefCell;
use super::memory::Memory;

struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
}

enum RegistersEnum {
    A, X, Y, PC, S, P,
}

impl Registers {
    fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
        }
    }

    fn get_a(&self) -> u8 {
        self.a
    }

    fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    fn get_x(&self) -> u8 {
        self.x
    }

    fn set_x(&mut self, x: u8) {
        self.x = x;
    }

    fn get_y(&self) -> u8 {
        self.y
    }

    fn set_y(&mut self, y: u8) {
        self.y = y;
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn set_pc(&self, pc: u16) {
        self.pc = pc;
    }

    fn reset_pc(&mut self) {
        self.pc = 0;
    }

    fn change_pc(&mut self, change: u16) {
        self.pc += change;
    }
}

struct FlagsRegister {
    negative: bool,
    overflow: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

const NEGATIVE_FLAG_BYTE_POSITION: u8 = 7;
const OVERFLOW_FLAG_BYTE_POSITION: u8 = 6;
const DECIMAL_FLAG_BYTE_POSITION: u8 = 3;
const INTERRUPT_FLAG_BYTE_POSITION: u8 = 2;
const ZERO_FLAG_BYTE_POSITION: u8 = 1;
const CARRY_FLAG_BYTE_POSITION: u8 = 0;

impl FlagsRegister {
    fn new() -> FlagsRegister {
        FlagsRegister {
            negative: false,
            overflow: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    fn set_negative(&mut self, value: bool) {
        self.negative = value;
    }

    fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    fn set_decimal(&mut self, value: bool) {
        self.decimal = value;
    }

    fn set_interrupt(&mut self, value: bool) {
        self.interrupt = value;
    }

    fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }
    
    fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    fn get_negative(&self) -> bool {
        self.negative
    }

    fn get_overflow(&self) -> bool {
        self.overflow
    }

    fn get_decimal(&self) -> bool {
        self.decimal
    }

    fn get_interrupt(&self) -> bool {
        self.interrupt
    }

    fn get_zero(&self) -> bool {
        self.zero
    }

    fn get_carry(&self) -> bool {
        self.carry
    }
}

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.negative   { 1 } else { 0 }) << NEGATIVE_FLAG_BYTE_POSITION |
        (if flag.overflow   { 1 } else { 0 }) << OVERFLOW_FLAG_BYTE_POSITION |
        (if flag.decimal    { 1 } else { 0 }) << DECIMAL_FLAG_BYTE_POSITION |
        (if flag.interrupt  { 1 } else { 0 }) << INTERRUPT_FLAG_BYTE_POSITION |
        (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION |
        (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> FlagsRegister {
        let negative = ((byte >> NEGATIVE_FLAG_BYTE_POSITION) & 0b1) != 0;
        let overflow = ((byte >> OVERFLOW_FLAG_BYTE_POSITION) & 0b1) != 0;
        let decimal = ((byte >> DECIMAL_FLAG_BYTE_POSITION) & 0b1) != 0;
        let interrupt = ((byte >> OVERFLOW_FLAG_BYTE_POSITION) & 0b1) != 0;
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> OVERFLOW_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            negative,
            overflow,
            decimal,
            interrupt,
            zero,
            carry,
        }
    }
}

pub enum Instruction {
    ADC(AddressingMode),
    AND(AddressingMode),
    ASL(AddressingMode),
    BCC(AddressingMode),
    BCS(AddressingMode),
    BEQ(AddressingMode),
    BIT(AddressingMode),
    BMI(AddressingMode),
    BNE(AddressingMode),
    BPL(AddressingMode),
    BRK(AddressingMode),
    BVC(AddressingMode),
    BVS(AddressingMode),
    CLC,
    CLD,
    CLI,
    CLV,
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    DEX,
    DEY,
    EOR(AddressingMode),
    INC(AddressingMode),
    INX,
    INY,
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP,
    ORA(AddressingMode),
    PHA(AddressingMode),
    PHP(AddressingMode),
    PLA(AddressingMode),
    PLP(AddressingMode),
    ROL(AddressingMode),
    ROR(AddressingMode),
    RTI(AddressingMode),
    RTS(AddressingMode),
    SBC(AddressingMode),
    SEC(AddressingMode),
    SED(AddressingMode),
    SEI(AddressingMode),
    STA(AddressingMode),
    STX(AddressingMode),
    STY(AddressingMode),
    TAX(AddressingMode),
    TAY(AddressingMode),
    TSX(AddressingMode),
    TXA(AddressingMode),
    TXS(AddressingMode),
    TYA(AddressingMode),
}

pub enum AddressingMode {
    Accumulator,
    Immediate(u8),
    ZeroPage(u8, u8),
    Relative(u8),
    Absolute(u16, u8),
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

struct ALU {
    flags_register: Rc<RefCell<FlagsRegister>>
}

// TODO: check signed values

impl ALU {
    fn new(flags_register: Rc<RefCell<FlagsRegister>>) -> ALU {
        ALU {
            flags_register,
        }
    }

    fn add_with_carry(&mut self, value1: u8, value2: u8) -> u8 {
        let carry: bool = (*self.flags_register.borrow_mut()).get_carry();
        let sum16: u16 = (value1 as u16) + (value2 as u16) + (if carry { 1 } else { 0 });
        let sum8: u8 = (sum16 & 0x00ff) as u8;
        let new_carry: bool = (((sum16 & 0x0100) >> 8) & 0b1) == 1;
        (*self.flags_register.borrow_mut()).set_carry(new_carry);
        (*self.flags_register.borrow_mut()).set_zero(sum8 == 0);
        // TODO: check this flag later
        (*self.flags_register.borrow_mut()).set_overflow(
            (((value1 >> 7) & 0b1) == 0 && ((value2 >> 7) & 0b1) == 0 && ((sum8 >> 7) & 0b1) == 1) |
            (((value1 >> 7) & 0b1) == 1 && ((value2 >> 7) & 0b1) == 1 && ((sum8 >> 7) & 0b1) == 0)
        );
        (*self.flags_register.borrow_mut()).set_negative(((sum8 >> 7) & 0b1) == 1);
        sum8
    }

    fn logical_and(&mut self, value1: u8, value2: u8) -> u8 {
        let and_result = value1 & value2;
        (*self.flags_register.borrow_mut()).set_zero(and_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((and_result >> 7) & 0b1) == 1);
        and_result
    }

    fn test_and(&mut self, value1: u8, value2: u8) {
        let and_result = value1 & value2;
        (*self.flags_register.borrow_mut()).set_zero(and_result == 0);
        (*self.flags_register.borrow_mut()).set_overflow(((and_result >> 6) & 0b1) == 1);
        (*self.flags_register.borrow_mut()).set_negative(((and_result >> 7) & 0b1) == 1);
    }

    fn logical_or(&mut self, value1: u8, value2: u8) -> u8 {
        let or_result = value1 | value2;
        (*self.flags_register.borrow_mut()).set_zero(or_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((or_result >> 7) & 0b1) == 1);
        or_result
    }

    fn logical_xor(&mut self, value1: u8, value2: u8) -> u8 {
        let xor_result = value1 ^ value2;
        (*self.flags_register.borrow_mut()).set_zero(xor_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((xor_result >> 7) & 0b1) == 1);
        xor_result
    }

    fn arithmetic_shift_left(&mut self, value1: u8) -> u8 {
        let shifted16 = (value1 as u16) * 2;
        let shift_result = (shifted16 & 0x00ff) as u8;
        let carry: bool = (((shifted16 & 0x0100) >> 8) & 0b1) == 1;
        (*self.flags_register.borrow_mut()).set_carry(carry);
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 8) & 0b1) == 1);
        shift_result
    }

    fn logical_shift_right(&mut self, value1: u8) -> u8 {
        let old_bit: bool = ((value1 & 0b1) == 1;
        let shift_result = value1 >> 1;
        (*self.flags_register.borrow_mut()).set_carry(old_bit);
        (*self.flags_register.borrow_mut()).set_zero(shift_result == 0);
        (*self.flags_register.borrow_mut()).set_negative(((shift_result >> 8) & 0b1) == 1);
        shift_result
    }

    fn compare(&mut self, value1: u8, value2: u8) {
        // TODO: check flag values
        (*self.flags_register.borrow_mut()).set_carry((value1 & 0x7f) > (value2 & 0x7f));
        (*self.flags_register.borrow_mut()).set_zero(value1 == value2);
        (*self.flags_register.borrow_mut()).set_negative((((value1 - value2) >> 7) & 0b1) == 1);
    }

    fn decrement(&mut self, value: u8) -> u8 {
        let new_value = value - 1;
        (*self.flags_register.borrow_mut()).set_zero(new_value == 0);
        (*self.flags_register.borrow_mut()).set_negative(((new_value >> 7) & 0b1) == 1);
        new_value
    }

    fn increment(&mut self, value: u8) -> u8 {
        let new_value = value + 1;
        (*self.flags_register.borrow_mut()).set_zero(new_value == 0);
        (*self.flags_register.borrow_mut()).set_negative(((new_value >> 7) & 0b1) == 1);
        new_value
    }
}

pub struct CPU {
    alu: ALU,
    registers: Registers,
    flags_register: Rc<RefCell<FlagsRegister>>,
    memory: Rc<RefCell<Memory>>
}

impl CPU {
    pub fn new(memory: Rc<RefCell<Memory>>) -> CPU {
        let registers = Registers::new();
        let flags_register: Rc<RefCell<FlagsRegister>> = Rc::new(RefCell::new(FlagsRegister::new()));
        let alu: ALU = ALU::new(Rc::clone(&flags_register));
        CPU {
            alu,
            registers,
            flags_register,
            memory,
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        match instruction {
            Instruction::ADC(addressing_mode) => {
                let (load_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value: u8 = self.registers.get_a();
                let sum_result: u8 = self.alu.add_with_carry(register_value, load_value);
                self.registers.set_a(sum_result);
                sum_result
            }

            Instruction::AND(addressing_mode) => {
                let (load_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value: u8 = self.registers.get_a();
                let and_result: u8 = self.alu.logical_and(load_value, register_value);
                self.registers.set_a(and_result);
                and_result
            }

            Instruction::ASL(addressing_mode) => {
                let (load_value, _address): (u8, u16)= self.resolve_addressing_mode(addressing_mode);
                let shift_result = self.alu.arithmetic_shift_left(load_value);
                self.registers.set_a(shift_result);
                shift_result
            }

            Instruction::BCC(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if !(*self.flags_register.borrow_mut()).get_carry() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BEQ(addressing_mode) => {
                let (load_offset, _address): (u8, u16)= self.resolve_addressing_mode(addressing_mode);
                if (*self.flags_register.borrow_mut()).get_zero() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BIT(addressing_mode) => {
                let (test_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value = self.registers.get_a();
                self.alu.test_and(test_value, register_value);
                0
            }

            Instruction::BMI(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if (*self.flags_register.borrow_mut()).get_negative() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BNE(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if !(*self.flags_register.borrow_mut()).get_zero() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BPL(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if !(*self.flags_register.borrow_mut()).get_negative() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BCS(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if (*self.flags_register.borrow_mut()).get_carry() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BRK(addressing_mode) => {
                // TODO: implement
                0
            }

            Instruction::BVC(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if !(*self.flags_register.borrow_mut()).get_overflow() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::BVS(addressing_mode) => {
                let (load_offset, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                if (*self.flags_register.borrow_mut()).get_overflow() {
                    self.registers.change_pc(load_offset as u16);
                }
                load_offset
            }

            Instruction::CLC => {
                (*self.flags_register.borrow_mut()).set_carry(false);
                0
            }

            Instruction::CLD => {
                (*self.flags_register.borrow_mut()).set_decimal(false);
                0
            }

            Instruction::CLI => {
                (*self.flags_register.borrow_mut()).set_interrupt(false);
                0
            }

            Instruction::CLV => {
                (*self.flags_register.borrow_mut()).set_overflow(false);
                0
            }

            Instruction::CMP(addressing_mode) => {
                let (compare_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value = self.registers.get_a();
                self.alu.compare(compare_value, register_value);
                0
            }

            Instruction::CPX(addressing_mode) => {
                let (compare_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value = self.registers.get_x();
                self.alu.compare(compare_value, register_value);
                0
            }

            Instruction::CPY(addressing_mode) => {
                let (compare_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value = self.registers.get_y();
                self.alu.compare(compare_value, register_value);
                0
            }

            Instruction::DEC(addressing_mode) => {
                let (to_dec_value, address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let decremented_value = self.alu.decrement(to_dec_value);
                (*self.memory.borrow_mut()).set_byte(address as usize, decremented_value);
                decremented_value
            }

            Instruction::DEX => {
                let register_value = self.registers.get_x();
                let decremented_value = self.alu.decrement(register_value);
                self.registers.set_x(decremented_value);
                decremented_value
            }

            Instruction::DEY => {
                let register_value = self.registers.get_y();
                let decremented_value = self.alu.decrement(register_value);
                self.registers.set_x(decremented_value);
                decremented_value
            }

            Instruction::EOR(addressing_mode) => {
                let (load_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value: u8 = self.registers.get_a();
                let xor_result: u8 = self.alu.logical_xor(load_value, register_value);
                self.registers.set_a(xor_result);
                xor_result
            }

            Instruction::INC(addressing_mode) => {
                let (to_inc_value, address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let incremented_value = self.alu.increment(to_inc_value);
                (*self.memory.borrow_mut()).set_byte(address as usize, incremented_value);
                incremented_value
            }

            Instruction::INX => {
                let register_value = self.registers.get_x();
                let incremented_value = self.alu.increment(register_value);
                self.registers.set_x(incremented_value);
                incremented_value
            }

            Instruction::INY => {
                let register_value = self.registers.get_y();
                let incremented_value = self.alu.increment(register_value);
                self.registers.set_x(incremented_value);
                incremented_value
            }

            Instruction::JMP(addressing_mode) => {
                let (_jump_address_value, jump_address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                self.registers.set_pc(jump_address);
                0
            }

            Instruction::JSR(addressing_mode) => {
                // TODO: implement stack
                0
            }

            Instruction::LDA(addressing_mode) => {
                let (load_value, _address) = self.resolve_addressing_mode(addressing_mode);
                self.registers.set_a(load_value);
                load_value
            }

            Instruction::LDX(addressing_mode) => {
                let (load_value, _address) = self.resolve_addressing_mode(addressing_mode);
                self.registers.set_x(load_value);
                load_value
            }

            Instruction::LDY(addressing_mode) => {
                let (load_value, _address) = self.resolve_addressing_mode(addressing_mode);
                self.registers.set_y(load_value);
                load_value
            }

            Instruction::LSR(addressing_mode) => {
                let (load_value, _address): (u8, u16)= self.resolve_addressing_mode(addressing_mode);
                let shift_result = self.alu.logical_shift_right(load_value);
                self.registers.set_a(shift_result);
                shift_result
            }

            Instruction::NOP => {
                0
            }

            Instruction::ORA(addressing_mode) => {
                let (load_value, _address): (u8, u16) = self.resolve_addressing_mode(addressing_mode);
                let register_value: u8 = self.registers.get_a();
                let or_result: u8 = self.alu.logical_or(load_value, register_value);
                self.registers.set_a(or_result);
                or_result
            }

            Instruction::PHA => {
                // TODO: implement stack
                0
            }

            Instruction::PHP => {
                // TODO: implement stack
                0
            }

            Instruction::PLA => {
                // TODO: implement stack
                0
            }

            Instruction::PLP => {
                // TODO: implement stack
                0
            }

            _ => { 0 }
        }
    }

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
                    register_value = self.registers.get_x();
                }
                // ZeroPageY
                if register == 2 {
                    register_value = self.registers.get_y();
                }
                let new_address: u8 = address + register_value;
                let address_value: u8 = (*self.memory.borrow_mut()).get_byte(new_address as usize);
                (address_value as u8, new_address as u16)
            }
            AddressingMode::Absolute(address, register) => {
                // Absolute
                let mut register_value: u16 = 0;
                // AbsoluteX
                if register == 1 {
                    register_value = self.registers.get_x() as u16;
                }
                // AbsoluteY
                if register == 2 {
                    register_value = self.registers.get_y() as u16;
                }
                let new_address: u16 = address + register_value;
                let address_value: u8 = (*self.memory.borrow_mut()).get_byte(new_address as usize);
                (address_value as u8, new_address as u16)
            }
            _ => { (0, 0) }
        }
    }
}