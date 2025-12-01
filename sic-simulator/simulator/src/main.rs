use std::io::{self, Write};

mod memory;
mod machine;
mod devices;
mod opcodes;

use machine::{Machine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    'reload: loop {
        let mut m = Machine::new();

        println!("Vnesi ime SIC objektne datoteke (.dev / .obj) iz mape programs:");
        print!("file> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let filename = line.trim();
        let path = format!("programs/{}", filename);

        if !path.is_empty() {
            match m.load_sic_object_file(&path) {
                Ok(()) => println!("Objektna datoteka naložena, PC = {:06X}", m.get_pc()),
                Err(e) => {
                    println!("Napaka pri nalaganju: {:?}", e);
                    continue 'reload;
                },
            }
        } else {
            println!("Napačno ime datoteke!");
            return Ok(());
        }

        println!("SIC/XE emulator");
        println!("Ukazi:");
        println!("  step              - izvedi en ukaz");
        println!("  run               - izvajaj do konca");
        println!("  regs              - izpiši registre");
        println!("  mem <addr> <len>  - izpiši pomnilnik");
        println!("  quit              - izhod");
        println!("  reset             - resetiraj simulator");

        let stdin = io::stdin();
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut line = String::new();
            if stdin.read_line(&mut line)? == 0 {
                break;
            }

            let mut parts = line.split_whitespace();
            let cmd = match parts.next() {
                Some(c) => c,
                None => continue,
            };

            match cmd {
                "step" => {
                    match m.step() {
                        Ok(()) => {}
                        Err(e) => {
                            println!("Napaka pri step: {:?}", e);
                        }
                    }
                }
                "reset" => {
                    continue 'reload
                }
                "run" => {
                    match m.run() {
                        Ok(()) => println!("Program končan (Interrupted)"),
                        Err(e) => println!("Program ustavljen z napako: {:?}", e),
                    }
                }
                "regs" => {
                    m.dump_registers();
                }
                "mem" => {
                    let start = parts.next().and_then(|s| usize::from_str_radix(s, 16).ok())
                        .unwrap_or(0);
                    let len = parts.next().and_then(|s| usize::from_str_radix(s, 16).ok())
                        .unwrap_or(0x40);
                    m.dump_mem(start, len);
                }
                "quit" | "exit" => return Ok(()),
                _ => {
                    println!("Neznan ukaz: {cmd}");
                }
            }
        }
    }
}
