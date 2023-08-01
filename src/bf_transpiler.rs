//! Brainfuck transpiler module, to convert from Brainfuck to any other language.

use crate::{code_memory::CodeMemory, tokenizer::CodeToken};

/// Brainfuck transpiler trait.
///
/// Given a specific code memory, transpiles the Brainfuck code into another
/// language like C.
pub trait BrainfuckTranspiler {
    /// Transpiles the Brainfuck code into another language.
    fn transpile(&self, code: CodeMemory) -> String;
}

/// Transpiles Brainfuck to C.
///
/// Example
/// ```rust
/// use bf::bf_transpiler::BrainfuckToCTranspiler;
/// use bf::{BrainfuckTranspiler, CodeMemory};
///
/// fn main() {
///     let code = CodeMemory::from_str("++++[>+++[>+<-]<-]>>;");
///     let transpiled = BrainfuckToCTranspiler::default().transpile(code);
///     println!("{transpiled}");
/// }
/// ```
pub struct BrainfuckToCTranspiler();

impl Default for BrainfuckToCTranspiler {
    fn default() -> Self {
        Self()
    }
}

impl BrainfuckTranspiler for BrainfuckToCTranspiler {
    fn transpile(&self, mut code: CodeMemory) -> String {
        let mut output = String::new();
        let mut stack = Vec::new();
        let mut stack_counter = 0_usize;

        output += "#include <stdio.h>\n";
        output += "#include <stdint.h>\n";
        output += "#include <string.h>\n";
        output += "\n";
        output += "const size_t memory_size = 128000;\n";
        output += "int64_t memory[memory_size];\n";
        output += "\n";
        output += "int main() {\n";
        output += "    memset(memory, 0, sizeof(int64_t) * memory_size);\n";
        output += "    int64_t* ptr = memory;\n";
        output += "    char tmp_char = '\\0';\n";

        while let Some(t) = code.read() {
            output += &format!("    // {} ({}, {})\n    ", t.token, t.column, t.line);
            let c = match t.token {
                CodeToken::Increment => "(*ptr)++;".into(),
                CodeToken::Decrement => "(*ptr)--;".into(),
                CodeToken::Next => "ptr++;".into(),
                CodeToken::Previous => "ptr--;".into(),
                CodeToken::StartLoop => {
                    let n = stack_counter;
                    stack.push(n);
                    stack_counter += 1;
                    format!("if(!(*ptr)) goto _jmp{n}_end;\n_jmp{n}_start:")
                }
                CodeToken::EndLoop => {
                    let n = stack.pop().unwrap();
                    format!("if(*ptr) goto _jmp{n}_start;\n_jmp{n}_end:")
                }
                CodeToken::Input => {
                    "fflush(stdout);\n    if(!fread(&tmp_char, sizeof(tmp_char), 1, stdin)) return 1;\n    *ptr = tmp_char;"
                        .into()
                }
                CodeToken::Output => "fputc((char) *ptr, stdout);".into(),
                #[cfg(feature = "with-debug")]
                CodeToken::Debug => format!("printf(\"dp = %llu d = %lli ({}, {})\", ptr - memory, *ptr);", t.column, t.line),
            };
            output += &c;
            output += "\n";
            code.next();
        }

        output += "    fflush(stdout);\n";
        output += "    return 0;\n";
        output += "}\n";
        output += "\n";

        output
    }
}
