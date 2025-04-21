// src/main.rs
mod parser;
mod assembler;
mod loader;
mod cpu;
mod dump;

use parser::Exec8Command;
use std::io::{self, Write, BufRead, BufReader};
use std::fs::File;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static MEMORY: Lazy<Mutex<Vec<u64>>> = Lazy::new(|| Mutex::new(vec![0; 1024]));
static REGISTERS: Lazy<Mutex<cpu::CpuState>> = Lazy::new(|| Mutex::new(cpu::CpuState::default()));

fn main() {
    println!("EXEC 8-lite Command Line Interface\n");

    loop {
        print!("READY > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("? INPUT ERROR");
            continue;
        }

        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("HALT") {
            println!("Halting system.");
            break;
        }

        let cmd = parser::parse_control_line(trimmed);
        dispatch_command(cmd);
    }
}

fn dispatch_command(cmd: Exec8Command) {
    println!("\n[Parsed EXEC 8 Command]");
    println!("Label: {:?}", cmd.label);
    println!("Command: {}", cmd.command);
    println!("Options: {:?}", cmd.options);
    println!("Parameters: {:?}", cmd.parameters);
    println!("Comment: {:?}\n", cmd.comment);

    match cmd.command.as_str() {
        "LOAD" => {
            if let Some(path) = cmd.parameters.get(0) {
                match loader::load_object_file(path) {
                    Ok(mem) => {
                        let mut memory = MEMORY.lock().unwrap();
                        for (i, word) in mem.iter().enumerate() {
                            memory[i] = *word;
                        }
                        println!("=> Memory loaded: {} words", mem.len());
                    }
                    Err(e) => println!("Loader error: {}", e),
                }
            } else {
                println!("? LOAD requires an object file path");
            }
        }
        "RUN" => {
            let mut memory = MEMORY.lock().unwrap();
            let mut regs = REGISTERS.lock().unwrap();
            cpu::run_program(&mut memory, &mut regs);
        }
        "STATUS" => println!("=> System Status: CPU ready, memory initialized"),
        "ASM" => {
            use assembler::assemble;
            if cmd.parameters.len() >= 2 {
                let source = &cmd.parameters[0];
                let output = &cmd.parameters[1];
                assemble(source, output).unwrap_or_else(|e| println!("Assembler error: {}", e));
            } else {
                println!("? ASM requires source and output parameters");
            }
        },
        "DUMP" => {
            let memory = MEMORY.lock().unwrap();
            let regs = REGISTERS.lock().unwrap();
            let start = cmd.parameters.get(0).and_then(|s| usize::from_str_radix(s, 8).ok()).unwrap_or(0);
            let count = cmd.parameters.get(1).and_then(|s| usize::from_str_radix(s, 8).ok()).unwrap_or(10);
            dump::dump_state(&memory, &regs, start, count);
        }
        "EXEC" => {
            if let Some(path) = cmd.parameters.get(0) {
                if let Ok(file) = File::open(path) {
                    let reader = BufReader::new(file);
                    for (i, line) in reader.lines().enumerate() {
                        if let Ok(command) = line {
                            if command.trim().is_empty() { continue; }
                            println!("\n> {}", command);
                            let parsed = parser::parse_control_line(&command);
                            dispatch_command(parsed);
                        } else {
                            println!("? Error reading line {} from {}", i + 1, path);
                        }
                    }
                } else {
                    println!("? Could not open EXEC file: {}", path);
                }
            } else {
                println!("? EXEC requires a filename");
            }
        }
        _ => println!("? UNKNOWN COMMAND"),
    }
}