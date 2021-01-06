pub struct Memory {
    bytes: [u8; 64 * 1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; 64 * 1024],
        }
    }

    pub fn get_byte(&mut self, location: u16) -> u8 {
        self.bytes[location as usize]
    }

    pub fn set_byte(&mut self, location: u16, value: u8) {
        self.bytes[location as usize] = value;
    }
}