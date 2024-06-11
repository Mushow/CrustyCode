mod crustycode;

use std::env;

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format!("Usage: ./{} <binary>", PROGRAM_NAME).into());
    }

    let file_path = &args[1];
    crustycode::run(file_path);

    Ok(())
}
