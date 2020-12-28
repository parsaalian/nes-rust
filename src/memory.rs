pub struct Memory {
    bytes: [u8; 64 * 1024],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            bytes: [0; 64 * 1024],
        }
    }

    pub fn get_byte(&mut self, location: usize) -> u8 {
        self.bytes[location]
    }

    pub fn set_byte(&mut self, location: usize, value: u8) {
        self.bytes[location] = value;
    }
}