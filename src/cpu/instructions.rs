use std::marker::Copy;
use std::collections::HashMap;

pub struct InstructionReader {
    instruction_map: HashMap<String, (String, String)>,
}

impl InstructionReader {
    pub fn new() -> Self {
        // from https://gist.github.com/kirbyUK/1a0797e19f54c1e35e67ce7b385b323e
        let instruction_opcodes: Vec<String> = vec![
            "69", "65", "75", "6D", "7D", "79", "61", "71", "29", "25", "35", "2D", "3D", "39", "21", "31",
            "0A", "06", "16", "0E", "1E", "90", "B0", "F0", "24", "2C", "30", "D0", "10", "00", "50", "70",
            "18", "D8", "58", "B8", "C9", "C5", "D5", "CD", "DD", "D9", "C1", "D1", "E0", "E4", "EC", "C0",
            "C4", "CC", "C6", "D6", "CE", "DE", "CA", "88", "49", "45", "55", "4D", "5D", "59", "41", "51",
            "E6", "F6", "EE", "FE", "E8", "C8", "4C", "6C", "20", "A9", "A5", "B5", "AD", "BD", "B9", "A1",
            "B1", "A2", "A6", "B6", "AE", "BE", "A0", "A4", "B4", "AC", "BC", "4A", "46", "56", "4E", "5E",
            "EA", "09", "05", "15", "0D", "1D", "19", "01", "11", "48", "08", "68", "28", "2A", "26", "36",
            "2E", "3E", "6A", "66", "76", "6E", "7E", "40", "60", "E9", "E5", "F5", "ED", "FD", "F9", "E1",
            "F1", "38", "F8", "78", "85", "95", "8D", "9D", "99", "81", "91", "86", "96", "8E", "84", "94",
            "8C", "AA", "A8", "BA", "8A", "9A", "98",
        ].into_iter().map(|x| String::from(x)).collect();

        let instruction_names_and_modes: Vec<(String, String)> = vec![
            ("ADC", "Immediate"),    ("ADC", "ZeroPage"),     ("ADC", "ZeroPage,X"),   ("ADC", "Absolute"),
            ("ADC", "Absolute,X"),   ("ADC", "Absolute,Y"),   ("ADC", "(Indirect,X)"), ("ADC", "(Indirect),Y"),
            ("AND", "Immediate"),    ("AND", "ZeroPage"),     ("AND", "ZeroPage,X"),   ("AND", "Absolute"),
            ("AND", "Absolute,X"),   ("AND", "Absolute,Y"),   ("AND", "(Indirect,X)"), ("AND", "(Indirect),Y"),
            ("ASL", "Accumulator"),  ("ASL", "ZeroPage"),     ("ASL", "ZeroPage,X"),   ("ASL", "Absolute"),
            ("ASL", "Absolute,X"),   ("BCC", "Relative"),     ("BCS", "Relative"),     ("BEQ", "Relative"),
            ("BIT", "ZeroPage"),     ("BIT", "Absolute"),     ("BMI", "Relative"),     ("BNE", "Relative"),
            ("BPL", "Relative"),     ("BRK", "Implied"),      ("BVC", "Relative"),     ("BVS", "Relative"),
            ("CLC", "Implied"),      ("CLD", "Implied"),      ("CLI", "Implied"),      ("CLV", "Implied"),
            ("CMP", "Immediate"),    ("CMP", "ZeroPage"),     ("CMP", "ZeroPage,X"),   ("CMP", "Absolute"),
            ("CMP", "Absolute,X"),   ("CMP", "Absolute,Y"),   ("CMP", "(Indirect,X)"), ("CMP", "(Indirect),Y"),
            ("CPX", "Immediate"),    ("CPX", "ZeroPage"),     ("CPX", "Absolute"),     ("CPY", "Immediate"),
            ("CPY", "ZeroPage"),     ("CPY", "Absolute"),     ("DEC", "ZeroPage"),     ("DEC", "ZeroPage,X"),
            ("DEC", "Absolute"),     ("DEC", "Absolute,X"),   ("DEX", "Implied"),      ("DEY", "Implied"),
            ("EOR", "Immediate"),    ("EOR", "ZeroPage"),     ("EOR", "ZeroPage,X"),   ("EOR", "Absolute"),
            ("EOR", "Absolute,X"),   ("EOR", "Absolute,Y"),   ("EOR", "(Indirect,X)"), ("EOR", "(Indirect),Y"),
            ("INC", "ZeroPage"),     ("INC", "ZeroPage,X"),   ("INC", "Absolute"),     ("INC", "Absolute,X"),
            ("INX", "Implied"),      ("INY", "Implied"),      ("JMP", "Absolute"),     ("JMP", "Indirect"),
            ("JSR", "Absolute"),     ("LDA", "Immediate"),    ("LDA", "ZeroPage"),     ("LDA", "ZeroPage,X"),
            ("LDA", "Absolute"),     ("LDA", "Absolute,X"),   ("LDA", "Absolute,Y"),   ("LDA", "(Indirect,X)"),
            ("LDA", "(Indirect),Y"), ("LDX", "Immediate"),    ("LDX", "ZeroPage"),     ("LDX", "ZeroPage,Y"),
            ("LDX", "Absolute"),     ("LDX", "Absolute,Y"),   ("LDY", "Immediate"),    ("LDY", "ZeroPage"),
            ("LDY", "ZeroPage,X"),   ("LDY", "Absolute"),     ("LDY", "Absolute,X"),   ("LSR", "Accumulator"),
            ("LSR", "ZeroPage"),     ("LSR", "ZeroPage,X"),   ("LSR", "Absolute"),     ("LSR", "Absolute,X"),
            ("NOP", "Implied"),      ("ORA", "Immediate"),    ("ORA", "ZeroPage"),     ("ORA", "ZeroPage,X"),
            ("ORA", "Absolute"),     ("ORA", "Absolute,X"),   ("ORA", "Absolute,Y"),   ("ORA", "(Indirect,X)"),
            ("ORA", "(Indirect),Y"), ("PHA", "Implied"),      ("PHP", "Implied"),      ("PLA", "Implied"),
            ("PLP", "Implied"),      ("ROL", "Accumulator"),  ("ROL", "ZeroPage"),     ("ROL", "ZeroPage,X"),
            ("ROL", "Absolute"),     ("ROL", "Absolute,X"),   ("ROR", "Accumulator"),  ("ROR", "ZeroPage"),
            ("ROR", "ZeroPage,X"),   ("ROR", "Absolute"),     ("ROR", "Absolute,X"),   ("RTI", "Implied"),
            ("RTS", "Implied"),      ("SBC", "Immediate"),    ("SBC", "ZeroPage"),     ("SBC", "ZeroPage,X"),
            ("SBC", "Absolute"),     ("SBC", "Absolute,X"),   ("SBC", "Absolute,Y"),   ("SBC", "(Indirect,X)"),
            ("SBC", "(Indirect),Y"), ("SEC", "Implied"),      ("SED", "Implied"),      ("SEI", "Implied"),
            ("STA", "ZeroPage"),     ("STA", "ZeroPage,X"),   ("STA", "Absolute"),     ("STA", "Absolute,X"),
            ("STA", "Absolute,Y"),   ("STA", "(Indirect,X)"), ("STA", "(Indirect),Y"), ("STX", "ZeroPage"),
            ("STX", "ZeroPage,Y"),   ("STX", "Absolute"),     ("STY", "ZeroPage"),     ("STY", "ZeroPage,X"),
            ("STY", "Absolute"),     ("TAX", "Implied"),      ("TAY", "Implied"),      ("TSX", "Implied"),
            ("TXA", "Implied"),      ("TXS", "Implied"),      ("TYA", "Implied"),
        ].into_iter().map(|x| (String::from(x.0), String::from(x.1))).collect();

        InstructionReader {
            instruction_map: instruction_opcodes.into_iter().zip(instruction_names_and_modes.into_iter()).collect()
        }
    }

    pub fn read(&mut self, s: &str) -> Instruction {
        let padded_string = format!("{:0<6}", s);
        let opcode = (&padded_string[0..2]).to_string();
        let operand1 = u16::from_str_radix(&padded_string[2..4], 16).unwrap();
        let operand2 = u16::from_str_radix(&padded_string[4..6], 16).unwrap();

        let (inst_name, inst_mode) = &self.instruction_map[&opcode];
        let instruction: InstructionType = inst_name.to_string().parse().unwrap();
        let address = self.mode_to_enum(inst_mode, operand1, operand2);

        Instruction {
            instruction,
            address,
        }
    }

    fn mode_to_enum(&self, mode: &str, op1: u16, op2: u16) -> AddressingMode {
        match mode {
            "Accumulator" =>  { AddressingMode::Accumulator }
            "Immediate" =>    { AddressingMode::Immediate(op1 as u8) }
            "ZeroPage" =>     { AddressingMode::ZeroPage(op1 as u8, 0) }
            "ZeroPage,X" =>   { AddressingMode::ZeroPage(op1 as u8, 1) }
            "ZeroPage,Y" =>   { AddressingMode::ZeroPage(op1 as u8, 2) }
            "Relative" =>     { AddressingMode::Relative(op1 as u8) }
            "Absolute" =>     { AddressingMode::Absolute(op1 * 256 + op2, 0) }
            "Absolute,X" =>   { AddressingMode::Absolute(op1 * 256 + op2, 1) }
            "Absolute,Y" =>   { AddressingMode::Absolute(op1 * 256 + op2, 2) }
            "Indirect" =>     { AddressingMode::Indirect(op1 * 256 + op2) }
            "(Indirect,X)" => { AddressingMode::IndexedIndirect(op1 as u8) }
            "(Indirect),Y" => { AddressingMode::IndirectIndexed(op1 as u8) }
            _ =>              { AddressingMode::Implied }
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    instruction: InstructionType,
    address: AddressingMode,
}

impl Instruction {
    pub fn get_value(&self) -> InstructionType {
        self.instruction
    }

    pub fn get_address(&self) -> AddressingMode {
        self.address
    }
}

// TODO: unofficial opcodes http://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes
custom_derive! {
    #[derive(Debug, EnumFromStr, Copy, Clone)]
    pub enum InstructionType {
        // LoadStoreInstructions
        LDA, LDX, LDY, STA, STX, STY,
        // RegisterTransferInstructions
        TAX, TAY, TXA, TYA,
        // StackOperationInstructions
        TSX, TXS, PHA, PHP, PLA, PLP,
        // LogicalInstructions
        AND, EOR, ORA, BIT,
        // ArithmeticsInstructions
        ADC, SBC, CMP, CPX, CPY,
        // IncrementDecrementInstructions
        INC, INX, INY, DEC, DEX, DEY,
        // ShiftInstructions
        ASL, LSR, ROL, ROR,
        // JumpCallInstructions
        JMP, JSR, RTS,
        // BranchInstructions
        BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS,
        // StatusFlagInstructions
        CLC, CLD, CLI, CLV, SEC, SED, SEI,
        // SystemFunctionsInstructions
        BRK, NOP, RTI,
    }
}

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate(u8),
    ZeroPage(u8, u8),
    Relative(u8),
    Absolute(u16, u8),
    Indirect(u16),
    IndexedIndirect(u8),
    IndirectIndexed(u8),
}