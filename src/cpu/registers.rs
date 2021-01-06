pub struct Registers {
    a: u8, x: u8, y: u8, pc: u16, s: u8, p: u8,
}

pub enum RegisterType {
    A, X, Y, PC, S, P,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
        }
    }

    pub fn get_register(&self, reg: RegisterType) -> u8 {
        match reg {
            RegisterType::A => self.a,
            RegisterType::X => self.x,
            RegisterType::Y => self.y,
            RegisterType::S => self.s,
            RegisterType::P => self.p,
            _ => panic!(),
        }
    }

    pub fn set_register(&mut self, reg: RegisterType, value: u8) {
        match reg {
            RegisterType::A => {
                self.a = value;
            }
            RegisterType::X => {
                self.x = value;
            }
            RegisterType::Y => {
                self.y = value;
            }
            RegisterType::S => {
                self.s = value;
            }
            RegisterType::P => {
                self.p = value;
            }
            _ => panic!(),
        }
    }

    pub fn get_pc(&mut self) -> u16 {
        self.pc
    }

    pub fn set_pc(&mut self, pc: u16) {
        self.pc = pc;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn change_pc(&mut self, change: u16) {
        self.pc += change;
    }
}

const NEGATIVE_FLAG_BYTE_POSITION: u8 = 7;
const OVERFLOW_FLAG_BYTE_POSITION: u8 = 6;
const DECIMAL_FLAG_BYTE_POSITION: u8 = 3;
const INTERRUPT_FLAG_BYTE_POSITION: u8 = 2;
const ZERO_FLAG_BYTE_POSITION: u8 = 1;
const CARRY_FLAG_BYTE_POSITION: u8 = 0;

pub struct FlagsRegister {
    negative: bool,
    overflow: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,
}

impl FlagsRegister {
    pub fn new() -> FlagsRegister {
        FlagsRegister {
            negative: false,
            overflow: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
        }
    }

    pub fn set_negative(&mut self, value: bool) {
        self.negative = value;
    }

    pub fn set_overflow(&mut self, value: bool) {
        self.overflow = value;
    }

    pub fn set_decimal(&mut self, value: bool) {
        self.decimal = value;
    }

    pub fn set_interrupt(&mut self, value: bool) {
        self.interrupt = value;
    }

    pub fn set_zero(&mut self, value: bool) {
        self.zero = value;
    }
    
    pub fn set_carry(&mut self, value: bool) {
        self.carry = value;
    }

    pub fn get_negative(&self) -> bool {
        self.negative
    }

    pub fn get_overflow(&self) -> bool {
        self.overflow
    }

    pub fn get_decimal(&self) -> bool {
        self.decimal
    }

    pub fn get_interrupt(&self) -> bool {
        self.interrupt
    }

    pub fn get_zero(&self) -> bool {
        self.zero
    }

    pub fn get_carry(&self) -> bool {
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