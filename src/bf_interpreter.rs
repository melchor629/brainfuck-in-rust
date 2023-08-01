use std::io::Result;

use crate::{
    code_memory::CodeMemory, data_memory::DataMemory, io_system::IOSystem, tokenizer::CodeToken,
};

/// Interpreter for the Brainfuck language.
///
/// Given a code memory and data memory, runs the code directly. If any IO
/// operations fail, the interpreter will fail. Code and Memory cannot be reused.
///
/// Example
/// ```rust
/// use bf::{BrainfuckInterpreter, CodeMemory, StandardIOSystem};
///
/// fn main() -> std::io::Result<()> {
///     let code = CodeMemory::from_str("++++[>+++[>+<-]<-]>>;");
///     let io = StandardIOSystem::default();
///     let mut interpreter = BrainfuckInterpreter::new(io);
///
///     interpreter.run(code, Default::default())?;
///
///     Ok(())
/// }
/// ```
pub struct BrainfuckInterpreter<IO: IOSystem> {
    io: IO,
}

impl<IO: IOSystem> BrainfuckInterpreter<IO> {
    /// Creates a new Brainfuck interpreter with the given `IO`.
    pub fn new(io: IO) -> Self {
        Self { io }
    }

    /// Runs the given interpreter with the given code memory and data memory.
    pub fn run(&mut self, mut code: CodeMemory, mut memory: DataMemory) -> Result<()> {
        let mut stack = Vec::<usize>::with_capacity(16);

        while let Some(t) = code.read() {
            match t.token {
                CodeToken::Increment => memory.increment(),
                CodeToken::Decrement => memory.decrement(),
                CodeToken::Next => memory.next(),
                CodeToken::Previous => memory.previous(),
                CodeToken::StartLoop => {
                    if memory.read() == 0 {
                        code.skip_loop();
                    } else {
                        stack.push(code.position());
                    }
                }
                CodeToken::EndLoop => {
                    if memory.read() != 0 {
                        code.move_to(*stack.last().unwrap());
                    } else {
                        stack.pop();
                    }
                }
                CodeToken::Output => self.io.write(memory.read())?,
                CodeToken::Input => memory.write(self.io.read()?),
                #[cfg(feature = "with-debug")]
                CodeToken::Debug => {
                    eprint!(
                        "dp = {} d = {} cp = {} i = {} ({}, {})",
                        memory.position(),
                        memory.read(),
                        code.position(),
                        t.token,
                        t.line,
                        t.column,
                    );
                    self.io.read().unwrap();
                }
            };

            code.next();
        }

        Ok(())
    }
}
