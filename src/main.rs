use std::{env, fs};
use std::fs::File;
use std::io::Read;

use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::info;
use crate::devices::output::read_output;

use crate::cpu::CPU;

pub mod cpu;
pub mod devices;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("No file found");
    let metadata = fs::metadata(&filename).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize] ;
    f.read(&mut buffer).expect("Buffer overflow");

    buffer
}

fn setup_logging() {
    let colors = ColoredLevelConfig::new()
        .debug(Color::BrightBlack)
        .info(Color::BrightBlue);

    fern::Dispatch::new()
        .chain(std::io::stdout())
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                // This will color the log level only, not the whole line. Just a touch.
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .apply()
        .unwrap();
}

fn main() {
    let mut cpu = CPU::new();
    let args: Vec<String> = env::args().collect();
    let mut rom: Vec<u8> = Vec::from([
        0xA9, 0x51, 0x8D, 0x00, 0x20
    ]);
    setup_logging();
    if args.len() >= 2 {
        println!("Loading {} ROM", &args[1]);
        rom = get_file_as_byte_vec(&args[1]);
    } else {
        println!("No ROM file specified, loading default ROM");
    }
    cpu.load(rom);
    cpu.execute();
    read_output(&mut cpu);
    cpu.reset();
}
