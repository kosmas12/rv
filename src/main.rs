use std::fs;
use std::env;


mod decoder;
use decoder::{decode, Instruction};

mod executer;
use executer::exec;

mod system;
use system::{RegisterFile, Memory};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name: String = {
        match args.len() {
            2 => {
                args[1].parse().unwrap()
            },
            _ => {
                panic!("Usage: {:} FILE", args[0].parse::<String>().unwrap());
            }
        }
    };

    let mut register_file: RegisterFile = RegisterFile::default();
    let mut memory: Memory = Memory::default(fs::read(file_name).unwrap());
    register_file.write(2, (memory.ram_base + memory.ram.len()) as u32);
    register_file.pc = memory.rom_base as u32;

    loop {
        let inst = decode(memory.read_word(register_file.pc as usize));
        let inst_ = decode(memory.read_word(register_file.pc as usize));
        //println!("PC: 0x{:X} Instruction: {:?}, {:}, {:}", register_file.pc, inst, register_file.read(14), register_file.read(14));
        exec(&mut register_file, &mut memory, inst);

        if let Instruction::ECALL() = inst_ {
            break;
        }
        if let Instruction::EBREAK() = inst_ {
            break;
        }
    }
    println!("Done!");
}
