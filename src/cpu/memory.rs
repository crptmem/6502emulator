use log::{debug, error, info};

#[derive(Debug)]
pub struct Memory {
    pub content: [u8; u16::MAX as usize + 1],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            content: [0; u16::MAX as usize + 1],
        }
    }

    pub fn read(&mut self, address: usize) -> u8 {
        // debug!("Address to read: {:#06x}", address);
        if address > 0xFFFF {
            error!("Invalid address: {:#06x}", address);
            return 0;
        }
        return self.content[address];
    }

    pub fn write(&mut self, address: usize, value: u8) {
        // debug!("Address to write: {:#06x} data to write: {}", address, value);
        if address > 0xFFFF {
            error!("Invalid address: {:#06x}", address);
        }
        self.content[address] = value;
    }

    pub fn read_word(&self, index: usize) -> u16 {
        // Assuming memory is a slice of bytes
        let byte1 = self.content[index] as u16;
        let byte2 = self.content[index + 1] as u16;

        // Combine the bytes into a little-endian word
        byte1 | (byte2 << 8)
    }

    pub fn write_word(&mut self, index: usize, value: u16) {
        // Assuming memory is a slice of bytes
        self.content[index] = value as u8;
        self.content[index + 1] = (value >> 8) as u8;
    }
}