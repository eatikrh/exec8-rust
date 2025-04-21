// src/cpu.rs
#[derive(Default)]
pub struct CpuState {
    pub acc: u64,
    pub r: [usize; 8],
}

pub fn run_program(memory: &mut [u64], state: &mut CpuState) {
    let mut pc: usize = 0;

    loop {
        if pc >= memory.len() {
            println!("! PC out of bounds");
            break;
        }

        let word = memory[pc];
        let opcode = (word >> 27) & 0o777;
        let addr = if opcode == 0o106 {
            (word & 0o777_777) as usize
        } else {
            (word & 0o777_777_777) as usize
        };

        match opcode {
            0o100 => { // LDA
                state.acc = memory[addr];
                println!("LDA [{}] => A = {}", addr, state.acc);
            }
            0o101 => { // ADD
                state.acc += memory[addr];
                println!("ADD [{}] => A = {}", addr, state.acc);
            }
            0o102 => { // STA
                memory[addr] = state.acc;
                println!("STA [{}] <= A ({})", addr, state.acc);
            }
            0o103 => { // JMP
                println!("JMP to {}", addr);
                pc = addr;
                continue;
            }
            0o104 => { // SUB
                state.acc = state.acc.wrapping_sub(memory[addr]);
                println!("SUB [{}] => A = {}", addr, state.acc);
            }
            0o105 => { // JNZ
                if state.acc != 0 {
                    println!("JNZ taken to {}", addr);
                    pc = addr;
                    continue;
                } else {
                    println!("JNZ not taken; A = {}", state.acc);
                }
            }
            0o106 => { // LMJ
                let reg_num = ((word >> 23) & 0o17) as usize;
                state.r[reg_num] = pc + 1;
                println!("LMJ to {} (R{} = {})", addr, reg_num, pc + 1);
                pc = addr;
                continue;
            }
            0o107 => { // ISZ
                let value = ((memory[addr] as i64) + 1) & 0o777_777_777_777;
                memory[addr] = value as u64;
                println!("ISZ [{}] => {}", addr, value);
                if value == 0 {
                    pc += 1;
                    println!("ISZ skipped next instruction");
                }
            }
            0o110 => { // OUT
                let val = memory[addr];
                if val <= 0o777_777_777_777 {
                    print!("OUT [{}] => ", addr);
                    // try to decode as ASCII string if it's a block of characters
                    let mut s = String::new();
                    let mut i = addr;
                    while i < memory.len() {
                        let word = memory[i];
                        if word == 0 { break; }
                        let bytes = word.to_be_bytes();
                        for b in bytes.iter().rev().cloned() {
                            if b == 0 { continue; }
                            s.push(b as char);
                        }
                        i += 1;
                    }
                    if s.is_empty() {
                        println!("{}", val);
                    } else {
                        println!("{}", s);
                    }
                }
            }
            0o777 => {
                println!("HLT encountered. Halting.");
                break;
            }
            _ => {
                println!("? Unknown opcode {:o} at PC {}", opcode, pc);
                break;
            }
        }

        pc += 1;
    }
}
