use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Result;

pub fn load_object_file(path: &str) -> Result<Vec<u64>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut memory = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let word = u64::from_str_radix(&line.trim(), 8)?; // parse as octal
        memory.push(word);
    }

    println!("* Loaded {} words from {}", memory.len(), path);
    Ok(memory)
}
