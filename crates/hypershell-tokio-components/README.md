[![Crates.io](https://img.shields.io/crates/v/hypershell-tokio-components.svg)](https://crates.io/crates/hypershell-tokio-components)
[![Docs.rs](https://docs.rs/hypershell-tokio-components/badge.svg)](https://docs.rs/hypershell-tokio-components)

# hypershell-tokio-components: commands, files, and streams on Tokio

`hypershell-tokio-components` interprets the process, file, and stream syntax of [Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/), on
[Tokio](https://tokio.rs/). It runs external commands, simple and streaming, reads and writes files,
writes to standard output, and converts between bytes and the stream types that stages exchange.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

Its items sit in four modules:

- **`dsl`:** the core execution syntax `CoreExec`, and `ToTokioAsyncRead`, which turns a stage's
  input into a Tokio reader.
- **`components`:** `CanUpdateCommand`, which adds a program's arguments to a
  `tokio::process::Command`.
- **`providers`:** the providers for commands, files, standard output, and the stream adapters,
  the input dispatchers that pick an adapter by the type a stage receives, and the execution error
  types, collected in the `HypershellTokioProvider` bundle.
- **`types`:** the stream wrappers `TokioAsyncReadStream`, `FuturesAsyncReadStream`, and
  `FuturesStream`, and `ChildOutputStream`, a streamed command's output.

`HypershellNamespace` routes this crate's syntax for every context that joins it. The
[streams and input dispatch](https://contextgeneric.dev/docs/projects/hypershell/architecture/streams-and-input-dispatch) page explains how stages
exchange streams.
