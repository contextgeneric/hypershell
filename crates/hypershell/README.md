[![Crates.io](https://img.shields.io/crates/v/hypershell.svg)](https://crates.io/crates/hypershell)
[![Docs.rs](https://docs.rs/hypershell/badge.svg)](https://docs.rs/hypershell)

# Hypershell: shell-script-like programs as Rust types

Hypershell is a small language for writing shell-script-like programs in Rust, built with
[Context-Generic Programming (CGP)](https://contextgeneric.dev/), a language extension for Rust with
pluggable trait implementations at compile-time. A program is a Rust type, and the context that runs
it is its interpreter, so both the language's syntax and its meaning can be extended without editing
its crates. Hypershell is an experimental proof of concept, not a replacement for a shell.

**[Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Examples](https://contextgeneric.dev/docs/projects/hypershell/examples/) · [Repository](https://github.com/contextgeneric/hypershell)**

This crate is the one a program depends on, beside `cgp` and an async runtime. This program runs
`echo hello world!` and prints the output:

```rust
use hypershell::prelude::*;

pub type Program = hypershell! {
        SimpleExec<
            StaticArg<"echo">,
            WithStaticArgs["hello", "world!"],
        >
    |   StreamToStdout
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    HypershellCli
        .handle(PhantomData::<Program>, Vec::new())
        .await?;

    Ok(())
}
```

`Program` is a type alias, and `HypershellCli` is a context whose whole wiring is one line that
joins `HypershellNamespace`. The namespace routes each piece of syntax to the provider that
interprets it, and the compiler resolves those choices while it type-checks the program.

## What the crate holds

- **`prelude`:** what a program file imports: `cgp::prelude`, `PhantomData`, the handler traits and
  keys, the anyhow `Error`, every syntax type of
  [`hypershell-components`](https://docs.rs/hypershell-components), the `hypershell!` macro, and the
  two contexts.
- **`contexts`:** `HypershellCli`, a context with no fields for programs that run commands and
  handle files and streams, and `HypershellHttp`, whose `http_client` field supplies the client that
  HTTP requests use.
- **`namespaces`:** `HypershellNamespace`, which assembles the language from the backend crates and
  which a context of your own joins.
- **`providers`:** `HypershellErrorHandler`, the error wiring that raises each library's errors into
  the context's `anyhow::Error`.

The `hypershell!` macro expects the prelude in scope, and its expansion names the `cgp` crate, so a
program's crate depends on `cgp` as well. The library and small programs such as this one build on
stable Rust, and programs that nest others deeply can need nightly with the next-generation trait
solver, as the [repository README](https://github.com/contextgeneric/hypershell#installation)
describes.
