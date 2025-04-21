// src/assembler.rs
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use anyhow::{Result, bail};

pub fn assemble(source_file: &str, output_file: &str) -> Result<()> {
    let file = File::open(source_file)?;
    let reader = BufReader::new(file);

    let mut machine_code = Vec::new();
    let mut labels = HashMap::new();
    let mut pc = 0u16;

    // First pass: collect labels
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        let trimmed = if let Some(idx) = trimmed.find(';') {
            &trimmed[..idx]
        } else {
            trimmed
        }.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") { continue; }

        if let Some(colon_pos) = trimmed.find(':') {
            let label = trimmed[..colon_pos].trim();
            labels.insert(label.to_string(), pc);
            let rest = trimmed[colon_pos + 1..].trim();
            if !rest.is_empty() {
                pc += 1;
            }
        } else {
            pc += 1;
        }
    }

    let file = File::open(source_file)?;
    let reader = BufReader::new(file);
    pc = 0;

    // Second pass: assemble
    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        let trimmed = if let Some(idx) = trimmed.find(';') {
            &trimmed[..idx]
        } else {
            trimmed
        }.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") { continue; }

        let instruction = if let Some(colon_pos) = trimmed.find(':') {
            &trimmed[colon_pos + 1..].trim()
        } else {
            trimmed
        };

        let parts: Vec<&str> = instruction.split_whitespace().collect();
        if parts.is_empty() { continue; }

        let opcode_str = parts[0].to_uppercase();
        if opcode_str == "DEC" {
            if parts.len() < 2 {
                bail!("DEC directive requires a value");
            }
            let value = parts[1]
                .parse::<i64>()
                .map_err(|_| anyhow::anyhow!("Invalid DEC value: {}", parts[1]))?;
            machine_code.push(value as u64 & 0o777_777_777_777);
            pc += 1;
            continue;
        }

        let opcode = match opcode_str.as_str() {
            "LDA" => 0o100,
            "ADD" => 0o101,
            "STA" => 0o102,
            "JMP" => 0o103,
            "HLT" => 0o777,
            "SUB" => 0o104,
            "JNZ" => 0o105,
            "LMJ" => 0o106,
            "ISZ" => 0o107,
            "OUT" => 0o110,
            _ => bail!("Unknown opcode: {}", parts[0]),
        };

        let word = if opcode == 0o106 {
            if parts.len() < 3 {
                bail!("LMJ requires two operands: register, label");
            }

            let reg_str = parts[1].trim_end_matches(',');
            let addr_str = parts[2];

            let reg_val = reg_str
                .parse::<u64>()
                .map_err(|_| anyhow::anyhow!("Invalid register in LMJ: {}", reg_str))?;

            let addr_val = if let Some(addr) = labels.get(addr_str) {
                *addr as u64
            } else if let Ok(value) = addr_str.parse::<u64>() {
                value
            } else {
                bail!("Unknown label or value: {}", addr_str);
            };

            ((opcode as u64) << 27) | ((reg_val & 0o17) << 23) | (addr_val & 0o777_777)
        } else {
            let operand = if parts.len() > 1 {
                let op = parts[1];
                if let Some(addr) = labels.get(op) {
                    *addr as u64
                } else if let Ok(value) = op.parse::<u64>() {
                    value
                } else {
                    bail!("Unknown label or value: {}", op);
                }
            } else {
                0
            };

            ((opcode as u64) << 27) | (operand & 0o777_777_777)
        };

        machine_code.push(word);
        pc += 1;
    }

    let mut output = File::create(output_file)?;
    for word in machine_code {
        writeln!(output, "{:012o}", word)?;
    }

    println!("* Assembly complete: {} instructions written to {}", pc, output_file);
    Ok(())
}
