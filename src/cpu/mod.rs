use std::process::exit;
use log::{debug, info};
use crate::cpu::bus::Bus;

use crate::cpu::memory::Memory;

pub mod memory;
pub mod bus;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    // Accumulator
    x: u8,
    // Index
    y: u8,
    // Index
    n: u8,
    // Negative
    v: u8,
    // Overflow
    reserved: u8,
    b: u8,
    // Break,
    d: u8,
    // Decimal
    i: u8,
    // Interrupt disable
    z: u8,
    // Zero
    c: u8,
    // Carry
    stack_pointer: u16,
    program_counter: u16,
}

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub memory: Memory,
    pub bus: Bus,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers {
                a: 0,
                x: 0,
                y: 0,
                n: 0,
                v: 0,
                reserved: 0,
                b: 0,
                d: 0,
                i: 0,
                z: 0,
                c: 0,
                stack_pointer: 0xff,
                program_counter: 0,
            },
            bus: Bus {
                buffer_size: 256,
                output: 0x2000
            },
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        assert!(0x0600 + program.len() / 2 <= self.memory.content.len());
        self.memory.content[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        info!("Loaded program to 0x0600 - {:#06x}", 0x0600 + program.len());
    }

    pub fn execute(&mut self) {
        let mut start = 0x0600;
        debug!("Executing: 0xFFFC is {:#06x}", start);
        let mut ret_addr = 0x0600;
        for _ in 0..1024 {
            let opcode = self.memory.read((start + self.registers.program_counter) as usize);

            if opcode != 0x00 {
                //debug!("opcode: {:#04x} at {:#06x}", opcode, start + self.registers.program_counter);
            }

            self.registers.program_counter += 1;

            if opcode == 0x00 {
                //info!("BRK instruction");
                //exit(0);
            }
            if opcode == 0xEA {
                //debug!("NOP instruction");
            }

            if opcode == 0xA9 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} in A register", byte);
                self.registers.a = byte;
            }

            if opcode == 0x8D {
                let addr = self.memory.read_word((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} to memory address {:#06x}", self.registers.a, addr);
                self.memory.write(addr as usize, self.registers.a);
            }

            if opcode == 0x69 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                self.registers.x += byte;
            }

            if opcode == 0xE8 {
                self.registers.x += 1;
            }

            if opcode == 0xAA {
                self.registers.x = self.registers.a
            }

            if opcode == 0xA2 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                //debug!("Writing {:#04x} in X register", byte);
                self.registers.x = byte;
            }

            if opcode == 0xA0 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                //debug!("Writing {:#04x} in Y register", byte);
                self.registers.y = byte;
            }

            if opcode == 0x4C {
                let word = self.memory.read_word((start + self.registers.program_counter) as usize);
                debug!("Jumping to {:#06x} from {:#06x}", word, start + self.registers.program_counter);
                self.registers.program_counter = 0;
                start = word;
            }

            if opcode == 0x20 {
                let word = self.memory.read_word((start + self.registers.program_counter) as usize);
                debug!("JSR to {:#06x} from {:#06x}", word, start + self.registers.program_counter);
                ret_addr = start + self.registers.program_counter;
                self.registers.program_counter = 0;
                start = word;
            }

            if opcode == 0x60 {
                debug!("RTS to {:#06x}", ret_addr);
                self.registers.program_counter = 0;
                start = ret_addr;
            }
        }
        debug!("CPU registers: {:?}", self.registers);
    }

    pub fn reset(&mut self) {
        info!("Resetting CPU");
        self.registers = Registers {
            a: 0,
            x: 0,
            y: 0,
            n: 0,
            v: 0,
            reserved: 0,
            b: 0,
            d: 0,
            i: 0,
            z: 0,
            c: 0,
            stack_pointer: 0xff,
            program_counter: 0,
        };
    }
}