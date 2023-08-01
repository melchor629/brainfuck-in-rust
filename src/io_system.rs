#[cfg(not(feature = "no-std"))]
use std::io::{stdin, stdout, Read, Result, Write};

#[cfg(feature = "no-std")]
use alloc::boxed::Box;

#[cfg(feature = "no-std")]
use core::{fmt::Display, result::Result};

#[cfg(feature = "no-std")]
pub type IOResult<T> = Result<T, Box<dyn Display>>;
#[cfg(not(feature = "no-std"))]
pub type IOResult<T> = Result<T>;

/// Represents Input/Output operations for the Brainfuck state machine.
///
/// To abstract any IO operation, this trait is used. There is a default
/// implementation `StandardIOSystem` that uses `stdin` and `stdout` for IO.
pub trait IOSystem {
    /// Reads one element from the stream.
    fn read(&mut self) -> IOResult<i64>;
    /// Writes one element to the stream.
    fn write(&mut self, value: i64) -> IOResult<()>;
}

/// Default implementation of `IOSystem` which uses standard io.
///
/// Under the hood, uses `stdin` and `stdout` for input and output.
///
/// To create this, use:
/// ```rust
/// StandardIOSystem::default();
/// ```
#[cfg(not(feature = "no-std"))]
pub struct StandardIOSystem {
    tmp_buf: [u8; 1],
}

#[cfg(not(feature = "no-std"))]
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

#[cfg(not(feature = "no-std"))]
impl Default for StandardIOSystem {
    fn default() -> Self {
        Self { tmp_buf: [0_u8; 1] }
    }
}
