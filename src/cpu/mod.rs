use log::{debug, info};

use crate::cpu::memory::Memory;

pub mod memory;
pub mod instructions;

#[warn(dead_code)]
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
    pub cycles: u8,
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
                stack_pointer: 0,
                program_counter: 0,
            },
            cycles: 0,
            memory: Memory::new(),
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        //self.memory.content[0x0600..(0x0600 + program.len())].copy_from_slice(&program[..]);
        assert!(0x0600 + program.len() / 2 <= self.memory.content.len());

        for (index, chunk) in program.chunks(1).enumerate() {
            self.memory.content[0x0600 + index] = u16::from(chunk[0]);
            // debug!("excepting: {:#06x} in: {:#06x} real value: {:#06x}", u16::from(chunk[0]), 0x0600 + index, self.memory.content[0x0600 + index]);
        }
        self.memory.write(0xFFFC, 0x0600);
        info!("Loaded program to 0x0600 - {:#06x}", 0x0600 + program.len());
    }

    pub fn execute(&mut self) {
        let start = self.memory.read(0xFFFC);
        debug!("Executing: 0xFFFC is {:#06x}", start);
        for _ in 0..128 {
            let opcode = self.memory.read((start + self.registers.program_counter) as usize);
            self.registers.program_counter += 1;
            //debug!("opcode: {:#06x}", opcode);
            if opcode == 0x00 {
                //info!("BRK instruction");
                //exit(0);
            }
            if opcode == 0xEA {
                debug!("NOP instruction");
            }

            if opcode == 0xA9 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} in A register", byte);
                self.registers.program_counter += 1;
                self.registers.a = byte as u8;
            }

            if opcode == 0x8D {
                let addr = self.memory.read((start + self.registers.program_counter) as usize);
                self.registers.program_counter += 1;
                let addr2 = self.memory.read((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} to memory address {:#06x}", self.registers.a, addr.swap_bytes() + addr2);
                self.memory.write((addr.swap_bytes() + addr2) as usize, self.registers.a as u16);
            }

            if opcode == 0xA2 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} in X register", byte);
                self.registers.program_counter += 1;
                self.registers.x = byte as u8;
            }

            if opcode == 0xA0 {
                let byte = self.memory.read((start + self.registers.program_counter) as usize);
                debug!("Writing {:#04x} in Y register", byte);
                self.registers.program_counter += 1;
                self.registers.y = byte as u8;
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
            stack_pointer: 0,
            program_counter: 0,
        };
        self.cycles = 0;
    }
}