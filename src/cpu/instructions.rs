use log::{debug, warn};
use crate::cpu::CPU;

pub fn run_opcode(mut cpu: &mut CPU, opcode: u8, mut start: u16, ret_addr: u16) {
    let mut ret_addr = 0x0600;

    match opcode{
        0x00 => { },
        0xEA => { },
        0xA9 => {
            let byte = cpu.memory.read((start + cpu.registers.program_counter) as usize);
            debug!("Writing {:#04x} in A register", byte);
            cpu.registers.a = byte;
            cpu.registers.program_counter += 1;
        },
        0x8D => {
            let addr = cpu.memory.read_word((start + cpu.registers.program_counter) as usize);
            debug!("Writing {:#04x} to memory address {:#06x}", cpu.registers.a, addr);
            cpu.memory.write(addr as usize, cpu.registers.a);
            cpu.registers.program_counter += 2;
        },
        0x69 => {
            let byte = cpu.memory.read((start + cpu.registers.program_counter) as usize);
            cpu.registers.x += byte;
            cpu.registers.program_counter += 1;
        },
        0xE8 => {
            cpu.registers.x += 1;
        },
        0xAA => {
            cpu.registers.x = cpu.registers.a
        },
        0xA2 => {
            let byte = cpu.memory.read((start + cpu.registers.program_counter) as usize);
            //debug!("Writing {:#04x} in X register", byte);
            cpu.registers.x = byte;
            cpu.registers.program_counter += 1;
        },
        0xA0 => {
            let byte = cpu.memory.read((start + cpu.registers.program_counter) as usize);
            //debug!("Writing {:#04x} in Y register", byte);
            cpu.registers.y = byte;
            cpu.registers.program_counter += 1;
        },
        0x4C => {
            let word = cpu.memory.read_word((start + cpu.registers.program_counter) as usize);
            debug!("Jumping to {:#06x} from {:#06x}", word, start + cpu.registers.program_counter);
            cpu.registers.program_counter = 0;
            start = word;
            cpu.registers.program_counter += 2;
        },
        0x20 => {
            let word = cpu.memory.read_word((start + cpu.registers.program_counter) as usize);
            debug!("JSR to {:#06x} from {:#06x}", word, start + cpu.registers.program_counter);
            ret_addr = start + cpu.registers.program_counter;
            cpu.registers.program_counter = 0;
            start = word;
        },
        0x60 => {
            debug!("RTS to {:#06x}", ret_addr);
            cpu.registers.program_counter = 0;
            start = ret_addr;
        }
        _=> warn!("Unknown opcode {:#06x}", opcode),
    }
}