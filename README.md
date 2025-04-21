# exec8-rust

A minimalist simulation of the UNIVAC EXEC 8 operating environment, written in Rust.

This project started as a way to learn Rust through low-level systems programming, but quickly grew into a respectful tribute to the architecture and operator workflows of 1970s mainframes — especially the UNIVAC 1100 series.

---

##  Features

-  EXEC 8-style control cards: `@ASM`, `@LOAD`, `@RUN`, `@DUMP`, `@EXEC`
-  Octal memory model using 36-bit words
- Instruction set includes:
    - `LDA`, `ADD`, `STA`, `SUB`, `JMP`, `JNZ`, `HLT`
    - Subroutine support via `LMJ` + `JMP r`
    - ASCII and numeric output via custom `OUT` instruction
-  Core dump and register inspection (`@DUMP`)
-  Batch job processing (`@EXEC jobs/job1.txt`)
-  Clean modular Rust design (`cpu.rs`, `assembler.rs`, `dump.rs`)

---

##  Why This Project?

> “To better learn Rust… and to hear the distant hum of an imaginary card reader.”

This emulator is not cycle-accurate or bit-exact — but it captures the **spirit of EXEC 8**:
the control flow, job structure, octal arithmetic, and debugging workflow of classic UNIVAC systems.

---

## 🔧 Getting Started

```bash
cargo run
