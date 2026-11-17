[![Crates.io](https://img.shields.io/crates/v/hypershell-components.svg)](https://crates.io/crates/hypershell-components)
[![Docs.rs](https://docs.rs/hypershell-components/badge.svg)](https://docs.rs/hypershell-components)

# hypershell-components: the syntax and components of Hypershell

`hypershell-components` defines the abstract syntax of [Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/), and the CGP components that its
backend crates implement. Every piece of syntax is an empty struct that names an operation without
saying how it runs, so a backend crate can interpret it without this crate depending on any
backend.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

The crate is `no_std` and links `alloc`. Its items sit in four modules:

- **`dsl`:** the syntax a program writes: execution (`SimpleExec`, `StreamingExec`, `WithArgs`,
  `WithStaticArgs`, `FieldArgs`), arguments (`StaticArg`, `FieldArg`, `JoinArgs`,
  `UrlEncodeArg`), HTTP (`SimpleHttpRequest`, `StreamingHttpRequest`, the method markers,
  `WithHeaders`, `Header`, `WebSocket`), streams and files (`StreamToBytes`, `StreamToString`,
  `StreamToStdout`, `BytesToStream`, `BytesToString`, `ReadFile`, `WriteFile`), JSON
  (`EncodeJson`, `DecodeJson`), and control (`Pipe`, `Use`, `ConvertTo`).
- **`components`:** the components that extract a command, URL, string, or HTTP method from an
  argument, and the abstract types they produce.
- **`providers`:** the providers that need no backend, such as `HandlePipe`, `Call`, and the
  argument extractors, collected in the `HypershellBaseProvider` bundle.
- **`traits`:** `WrapCall` and `WrapStaticArg`, which map over type-level lists.

`hypershell::prelude` re-exports the whole `dsl` module. The
[abstract syntax](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax) page explains how a program becomes a type.
