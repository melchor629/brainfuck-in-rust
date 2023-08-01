use std::path::Path;

use bf::{BrainfuckInterpreter, CodeMemory, StandardIOSystem};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() <= 1 {
        eprintln!("Required argument: file.");
        std::process::exit(1);
    }

    let code = CodeMemory::from_file(Path::new(&args[1]))?;
    let io = StandardIOSystem::default();
    let mut interpreter = BrainfuckInterpreter::new(io);

    interpreter.run(code, Default::default())?;

    Ok(())
}
