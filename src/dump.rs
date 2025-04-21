// src/dump.rs
use crate::cpu::CpuState;

pub fn dump_state(memory: &[u64], regs: &CpuState, start: usize, count: usize) {
    println!("\nREGISTERS:");
    println!("A : {:012o}", regs.acc);
    for i in 0..8 {
        println!("R{}: {:06o}", i, regs.r[i]);
    }

    println!("\nMEMORY (from {:04o}):", start);
    for addr in start..start + count {
        if addr < memory.len() {
            println!("{:04o}: {:012o}", addr, memory[addr]);
        }
    }
}
