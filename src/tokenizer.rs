//! Tokenizer for the Brainfuck language.
//!
//! This module contains the tokenizer for the laguage with expanded information
//! about the original place of the token in the source code. Useful for
//! interpreting or transpiling the language.

use core::fmt::Display;

#[cfg(feature = "no-std")]
use alloc::{vec, vec::Vec};

const INCREMENT_TOKEN: u8 = '+' as u8;
const DECREMENT_TOKEN: u8 = '-' as u8;
const NEXT_TOKEN: u8 = '>' as u8;
const PREVIOUS_TOKEN: u8 = '<' as u8;
const START_LOOP_TOKEN: u8 = '[' as u8;
const END_LOOP_TOKEN: u8 = ']' as u8;
const OUTPUT_TOKEN: u8 = '.' as u8;
const INPUT_TOKEN: u8 = ',' as u8;
#[cfg(feature = "with-debug")]
const DEBUG_TOKEN: u8 = ';' as u8;
const NEW_LINE_TOKEN: u8 = '\n' as u8;

/// Represents a token from the Brainfuck language.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(u8)]
pub enum CodeToken {
    /// Increment operation token (+).
    Increment = INCREMENT_TOKEN,
    /// Decrement operation token (-).
    Decrement = DECREMENT_TOKEN,
    /// Next operation token (>).
    Next = NEXT_TOKEN,
    /// Previous operation token (<).
    Previous = PREVIOUS_TOKEN,
    /// Start loop operation token ([).
    StartLoop = START_LOOP_TOKEN,
    /// End loop operation token (]).
    EndLoop = END_LOOP_TOKEN,
    /// Read from input operation token (,).
    Input = INPUT_TOKEN,
    /// Write to output operation token (.).
    Output = OUTPUT_TOKEN,
    #[cfg(feature = "with-debug")]
    /// Debug operation token (;).
    Debug = DEBUG_TOKEN,
}

/// Represents a token with its position from source code.
#[derive(Debug, Copy, Clone)]
pub struct CodeDebugToken {
    /// Token
    pub token: CodeToken,
    /// Line where the token is located in source
    pub line: usize,
    /// Column where the token is located in source
    pub column: usize,
}

/// Given the source code of a Brainfuck program, returns the list of tokens
/// extracted from the code, with its positions for easy debug.
pub fn tokenize(bytes: &[u8]) -> Vec<CodeDebugToken> {
    let mut tokens = vec![];
    let mut line = 1;
    let mut column = 1;

    for c in bytes {
        if let Ok(token) = (*c).try_into() {
            tokens.push(CodeDebugToken {
                token,
                line,
                column,
            });
            column += 1;
        } else if *c == NEW_LINE_TOKEN {
            column = 1;
            line += 1;
        } else {
            column += 1;
        }
    }

    tokens
}

impl TryFrom<u8> for CodeToken {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            INCREMENT_TOKEN => Ok(CodeToken::Increment),
            DECREMENT_TOKEN => Ok(CodeToken::Decrement),
            NEXT_TOKEN => Ok(CodeToken::Next),
            PREVIOUS_TOKEN => Ok(CodeToken::Previous),
            START_LOOP_TOKEN => Ok(CodeToken::StartLoop),
            END_LOOP_TOKEN => Ok(CodeToken::EndLoop),
            INPUT_TOKEN => Ok(CodeToken::Input),
            OUTPUT_TOKEN => Ok(CodeToken::Output),
            #[cfg(feature = "with-debug")]
            DEBUG_TOKEN => Ok(CodeToken::Debug),
            _ => Err(()),
        }
    }
}

impl From<CodeToken> for u8 {
    fn from(value: CodeToken) -> Self {
        value as u8
    }
}

impl From<&CodeToken> for u8 {
    fn from(value: &CodeToken) -> Self {
        (*value).into()
    }
}

impl Display for CodeToken {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let code: u8 = self.into();
        write!(f, "{}", code as char)
    }
}
