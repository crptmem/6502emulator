use log::info;
use crate::cpu::CPU;

pub fn read_output(mut cpu: &mut CPU) {
    let mut output = String::from("");
    for i in cpu.bus.output as usize .. cpu.bus.output as usize + cpu.bus.buffer_size {
        let byte = cpu.memory.read(i);
        if byte != 0x00 {
            output = format!("{}{}", String::from(output), String::from((byte) as char));
        }
    }
    info!("{}", output);
}