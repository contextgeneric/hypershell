[![Crates.io](https://img.shields.io/crates/v/hypershell-macro.svg)](https://crates.io/crates/hypershell-macro)
[![Docs.rs](https://docs.rs/hypershell-macro/badge.svg)](https://docs.rs/hypershell-macro)

# hypershell-macro: the hypershell! macro

`hypershell-macro` provides the `hypershell!` procedural macro of [Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/). The macro rewrites a
shell-like surface into the Rust type that is the program: `|` becomes a `Pipe` of the stages,
a bracketed list becomes a type-level list, and each string literal becomes a type-level string.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

A program reaches the macro through `hypershell::prelude`, so it has no reason to depend on this
crate directly. The macro only rewrites tokens and resolves no names, so its expansion expects the
prelude in scope and names the `cgp` crate.
