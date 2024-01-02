use log::info;
use crate::cpu::CPU;

pub fn read_output(mut cpu: CPU) {
    let mut output = String::from("");
    for i in cpu.bus.output .. cpu.bus.output + cpu.bus.buffer_size {
        let byte = cpu.memory.read(i as usize);
        if byte != 0x00 {
            output = format!("{}{}", String::from(output), String::from((byte) as char));
        }
    }
    info!("{}", output);
}