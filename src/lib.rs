#![deny(missing_docs)]

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

mod bf_interpreter;
pub mod bf_transpiler;
mod code_memory;
mod data_memory;
mod io_system;
pub mod tokenizer;

pub use bf_interpreter::BrainfuckInterpreter;
pub use bf_transpiler::BrainfuckTranspiler;
pub use code_memory::CodeMemory;
pub use io_system::{IOSystem, StandardIOSystem};
