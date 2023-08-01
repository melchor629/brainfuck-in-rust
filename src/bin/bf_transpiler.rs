use std::{fs::File, io::Write, path::Path};

use bf::bf_transpiler::BrainfuckToCTranspiler;
use bf::{BrainfuckTranspiler, CodeMemory};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("Required argument: file.");
        std::process::exit(1);
    }

    if args.len() <= 2 {
        eprintln!("Required argument: output file.");
        std::process::exit(1);
    }

    let code = CodeMemory::from_file(Path::new(&args[1]))?;
    let output_path = Path::new(&args[2]);
    let extension = output_path
        .extension()
        .unwrap_or(std::ffi::OsStr::new(".c"))
        .to_str()
        .unwrap();
    let transpiled = match extension {
        "c" => BrainfuckToCTranspiler::default().transpile(code),
        _ => {
            eprintln!("Could not transpile to {}: unknown extension.", args[2]);
            std::process::exit(1);
        }
    };
    let mut output = File::create(output_path)?;
    output.write_all(transpiled.as_bytes())?;

    Ok(())
}
