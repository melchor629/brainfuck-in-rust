#[cfg(not(feature = "no-std"))]
use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

#[cfg(feature = "no-std")]
use alloc::vec::Vec;

use crate::tokenizer::{tokenize, CodeDebugToken, CodeToken};

/// Represents the memory of code in the Brainfuck state machine.
///
/// The code can be read from a `&str`, `&String`, `&[u8]` or from any file.
/// The constructor will parse the file and will store only the parsed tokens
/// to optimize execution.
///
/// The code can be only use once.
#[derive(Clone)]
pub struct CodeMemory {
    code: Vec<CodeDebugToken>,
    pointer: usize,
}

impl CodeMemory {
    /// Creates the memory from the given tokens.
    pub fn from_tokens(code: Vec<CodeDebugToken>) -> Self {
        Self { code, pointer: 0 }
    }

    /// Creates the memory from the source code in bytes.
    pub fn from_bytes(code: &[u8]) -> Self {
        Self::from_tokens(tokenize(code))
    }

    /// Creates the memory from the source code.
    pub fn from_str(code: &str) -> Self {
        Self::from_bytes(code.as_bytes())
    }

    /// Creates the memory from the source code.
    #[cfg(not(feature = "no-std"))]
    pub fn from_string(code: &String) -> Self {
        Self::from_bytes(code.as_bytes())
    }

    /// Creates the memory from the source code located in a file.
    #[cfg(not(feature = "no-std"))]
    pub fn from_file(path: &Path) -> Result<Self> {
        let mut file = File::open(&path)?;
        let mut string = String::new();
        file.read_to_string(&mut string)?;
        Ok((&string).into())
    }

    pub(crate) fn next(&mut self) {
        self.pointer += 1;
    }

    pub(crate) fn read(&self) -> Option<CodeDebugToken> {
        if self.pointer >= self.code.len() {
            None
        } else {
            Some(self.code[self.pointer])
        }
    }

    pub(crate) fn move_to(&mut self, position: usize) {
        self.pointer = position;
    }

    pub(crate) fn skip_loop(&mut self) {
        let mut ign = 0;
        loop {
            if self.pointer < self.code.len() {
                self.next();
                let t = self.code[self.pointer].token;
                if t == CodeToken::EndLoop {
                    if ign == 0 {
                        break;
                    } else {
                        ign -= 1;
                    }
                } else if t == CodeToken::StartLoop {
                    ign += 1;
                }
            } else {
                break;
            }
        }
    }

    pub(crate) fn position(&self) -> usize {
        self.pointer
    }
}

impl From<&str> for CodeMemory {
    fn from(value: &str) -> Self {
        Self::from_str(&value)
    }
}

#[cfg(not(feature = "no-std"))]
impl From<&String> for CodeMemory {
    fn from(value: &String) -> Self {
        Self::from_string(&value)
    }
}
