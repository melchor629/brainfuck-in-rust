# brainfuck in rust

Welcome 👋, welcome to this implementation of a [Brainfuck][bf] interpreter and transpiler. It is a
simple implementation that runs all the instructions of the language plus an extra `;` for debugging
purposes (behind `with-debug` feature).

This repo presents the interpreter and transpiler as a Rust library to be used in any project - why
someone would want to use it anyway? why did I made this?? It also includes two binaries: one to run
the interpreter over a file, the other to transpile to the supported languages.

Interpreter does not apply any optimizations, except the end to start loop jump. It could contain a
fold optimization for `+`, `-`, `>` and `<`, but this would not be funny.

Transpiler can only transpile to C currently. The output can be then compiled with your favourite
compiler, and the desired optimizations, and it will fly.

It also contains a parser (tokenizer) which converts known tokens into `CodeToken`s and discards the
rest of tokens. Because the language is so simple, it does not contain an AST. But probably with
one, loops/ifs would be easier to optimize in the interpreter.

There is another... Another feature that allows this library to run in `no_std` environments. The
only dependencies are with `core` and `alloc`.

Have fun :)

  [bf]: https://en.wikipedia.org/wiki/Brainfuck
