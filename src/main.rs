#![feature(int_to_from_bytes)]
extern crate RISC_16_bit;
extern crate modVM;
use RISC_16_bit::*;
use std::num::Wrapping;
use std::{process, env};
use std::fs::{read, read_to_string, write};
use std::path::Path;
mod compiler;
mod scc;

fn read_u16(bytestream: &[u8]) -> Vec<u16> {
    bytestream.chunks(2)
        .filter(|d| {
            d.len() == 2
        })
        .map(|d| {
            ((d[0] as u16) << 8) | d[1] as u16
        })
        .collect()
}

fn load(mem: &mut Box<[u16; 65536]>, program: &[u16], offset: u16) {
    for (index, value) in program.iter().enumerate() {
        let offset_pos = (Wrapping(index) + Wrapping(offset as usize)).0;
        mem[offset_pos] = *value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Argument ERROR: Not enough arguments supplied.");
        process::exit(1);
    }
    match args[1].as_str() {
        "compile" => {
            if args.len() < 4 {
                println!("Argument ERROR: Not enough arguments supplied.");
                process::exit(1);
            }
            match compiler::compile(Path::new(&args[2]), Path::new(&args[3])) {
                Ok(x) => {
                    println!("Finished Compilation.");
                    process::exit(0);
                },
                Err(x) => {
                    println!("Compilation ERROR: {}", x);
                    process::exit(2);
                },
            }
        },
        "run" => {
            let data = match read(&args[2]) {
                Ok(x) => read_u16(&x),
                Err(x) => {
                    println!("Application ERROR: {}", x);
                    process::exit(3);
                },
            };

            let mut mem = Box::new([0; 65536]);
            load(&mut mem, &data, 0);

            let memory = PrintMemory::from_data(mem);
            let processor = MainProcessor::new();

            println!("16BitRiscMachineSTART:=>");

            let machine = modVM::Machine::from(vec![Box::new(memory)], vec![Box::new(processor)]);

            machine.run().unwrap().join_processors();

            println!(":=>Machine Halted")
        },
        "scc" => {
            if args.len() < 4 {
                println!("Argument ERROR: Not enough arguments supplied.");
                process::exit(1);
            }
            let data = match read_to_string(&args[2]) {
                Ok(x) => x,
                Err(x) => {
                    println!("Application ERROR: {}", x);
                    process::exit(3);
                },
            };

            let compiled = match scc::compile(data) {
                Ok(x) => x,
                Err(x) => {
                    println!("Application ERROR: {}", x);
                    process::exit(3);
                },
            };

            match write(&args[3], compiled) {
                Ok(_) => {},
                Err(x) => {
                    println!("Application ERROR: {}", x);
                    process::exit(3);
                },
            };
        },
        _ => {
            println!("Argument ERROR: Command `{}` not recognised.", args[0]);
            process::exit(1);
        }
    }
}
