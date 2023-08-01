#![deny(missing_docs)]
#![cfg_attr(feature = "no-std", no_std)]

//! Brainfuck interpreter and transpiler.
//!
//! Utilities to parse and interpret or transpile Brainfuck code.
//!
//! ## Interpret example
//! ```rust
//! use bf::{BrainfuckInterpreter, CodeMemory, StandardIOSystem};
//!
//! let code = CodeMemory::from_str("++++[>+++[>+<-]<-]>>;");
//! let io = StandardIOSystem::default();
//! let mut interpreter = BrainfuckInterpreter::new(io);
//! let memory = Default::default();
//!
//! interpreter.run(code, memory)?;
//! ```
//!
//! ## Transpile example
//! ```rust
//! use bf::bf_transpiler::BrainfuckToCTranspiler;
//! use bf::{BrainfuckTranspiler, CodeMemory};
//!
//! let code = CodeMemory::from_str("++++[>+++[>+<-]<-]>>;");
//! let transpiled = BrainfuckToCTranspiler::default().transpile(code);
//! println!("{transpiled}");
//! ```
//!
//! ## Tokenize example
//! ```rust
//! use bf::tokenizer::tokenize;
//!
//! let code = "++++[>+++[>+<-]<-]>>;";
//! let tokens = tokenize(code.as_bytes());
//! ```

#[cfg(feature = "no-std")]
extern crate alloc;

mod bf_interpreter;
#[cfg(not(feature = "no-std"))]
pub mod bf_transpiler;
mod code_memory;
mod data_memory;
mod io_system;
pub mod tokenizer;

pub use bf_interpreter::BrainfuckInterpreter;
#[cfg(not(feature = "no-std"))]
pub use bf_transpiler::BrainfuckTranspiler;
pub use code_memory::CodeMemory;
#[cfg(feature = "no-std")]
pub use io_system::IOSystem;
#[cfg(not(feature = "no-std"))]
pub use io_system::{IOSystem, StandardIOSystem};
