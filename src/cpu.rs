struct Registers {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    s: u8,
    p: u8,
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
    CLC(AddressingMode),
    CLD(AddressingMode),
    CLI(AddressingMode),
    CLV(AddressingMode),
    CMP(AddressingMode),
    CPX(AddressingMode),
    CPY(AddressingMode),
    DEC(AddressingMode),
    DEX(AddressingMode),
    DEY(AddressingMode),
    EOR(AddressingMode),
    INC(AddressingMode),
    INX(AddressingMode),
    INY(AddressingMode),
    JMP(AddressingMode),
    JSR(AddressingMode),
    LDA(AddressingMode),
    LDX(AddressingMode),
    LDY(AddressingMode),
    LSR(AddressingMode),
    NOP(AddressingMode),
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
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

struct ALU {}

impl ALU {
    fn new() -> ALU {
        ALU {}
    }

    fn add_with_carry(&mut self, value1: u8, value2: u8, carry: bool) -> (u8, bool) {
        let sum16: u16 = (value1 as u16) + (value2 as u16) + (if carry { 1 } else { 0 });
        let sum8: u8 = (sum16 & 0x00ff) as u8;
        let new_carry: bool = (((sum16 & 0x0100) >> 8) & 0b1) == 1;
        (sum8, new_carry)
    }
}

pub struct CPU {
    alu: ALU,
    registers: Registers,
    flags_register: FlagsRegister,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            alu: ALU::new(),
            registers: Registers::new(),
            flags_register: FlagsRegister::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) -> u8 {
        match instruction {
            Instruction::ADC(addressing_mode) => {
                match addressing_mode {
                    AddressingMode::Immediate(value) => {
                        let (sum, carry) = self.alu.add_with_carry(self.registers.get_a(), value, self.flags_register.get_carry());
                        let negative = ((sum >> 7) & 0b1) == 1;
                        self.flags_register.set_carry(carry);
                        self.flags_register.set_negative(negative);
                        self.flags_register.set_zero(sum == 0);
                        sum
                    }
                    _ => { 0 }
                }
            }

            Instruction::LDA(addressing_mode) => {
                match addressing_mode {
                    AddressingMode::Immediate(value) => {
                        self.registers.set_a(value);
                        value
                    }
                    _ => { 0 }
                }
            }

            _ => { 0 }
        }
    }
}