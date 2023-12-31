use log::{error, info};

#[derive(Debug)]
pub struct Memory {
    pub content: [u16; 65535],
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            content: [0; 65535],
        }
    }

    pub fn read(&mut self, address: usize) -> u16 {
        // debug!("Address to read: {:#06x}", address);
        if address > 0xFFFF {
            error!("Invalid address: {:#06x}", address);
            return 0;
        }
        return self.content[address];
    }

    pub fn write(&mut self, address: usize, value: u16) {
        // debug!("Address to write: {:#06x} data to write: {}", address, value);
        if address > 0xFFFF {
            error!("Invalid address: {:#06x}", address);
        }
        self.content[address] = value;
    }

    pub fn read_output(&mut self) {
        let mut output = String::from("");
        for i in 0x1999..0x3400 {
            let byte = self.read(i);
            if byte != 0x00 {
                output = format!("{}{}", String::from(output), String::from((byte as u8) as char));
            }
        }
        info!("out: {}", output);
    }
}