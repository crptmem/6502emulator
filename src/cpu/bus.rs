#[derive(Debug)]
pub struct Bus {
    pub buffer_size: usize,
    pub output: u16
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            buffer_size: 256,
            output: 0x0000
        }
    }
}