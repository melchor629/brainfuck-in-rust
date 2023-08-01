use std::io::{stdin, stdout, Read, Result, Write};

/// Represents Input/Output operations for the Brainfuck state machine.
///
/// To abstractk any IO operation, this trait is used. There is a default
/// implementation `StandardIOSystem` that uses `stdin` and `stdout` for IO.
pub trait IOSystem {
    /// Reads one element from the stream.
    fn read(&mut self) -> Result<i64>;
    /// Writes one element to the stream.
    fn write(&mut self, value: i64) -> Result<()>;
}

/// Default implementation of `IOSystem` which uses standard io.
///
/// Under the hood, uses `stdin` and `stdout` for input and output.
///
/// To create this, use:
/// ```rust
/// StandardIOSystem::default();
/// ```
pub struct StandardIOSystem {
    tmp_buf: [u8; 1],
}

impl IOSystem for StandardIOSystem {
    fn read(&mut self) -> Result<i64> {
        stdin().read_exact(&mut self.tmp_buf)?;
        Ok(self.tmp_buf[0] as i64)
    }

    fn write(&mut self, value: i64) -> Result<()> {
        self.tmp_buf[0] = value as u8;
        stdout().write_all(&self.tmp_buf)
    }
}

impl Default for StandardIOSystem {
    fn default() -> Self {
        Self { tmp_buf: [0_u8; 1] }
    }
}
