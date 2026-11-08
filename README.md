![Crates.io](https://img.shields.io/crates/v/hypershell.svg)
[![Docs.rs](https://docs.rs/hypershell/badge.svg)](https://docs.rs/hypershell)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/contextgeneric/hypershell/blob/main/LICENSE)

# Hypershell: a type-level DSL for shell scripting in Rust

**A modular, type-level domain-specific language for writing shell-script-like programs in Rust,
powered by [Context-Generic Programming (CGP)](https://contextgeneric.dev/).**

CGP makes it possible to extend or modify both the language's syntax and its semantics, so a program
can mix external commands with native Rust handlers and still be checked by the compiler.

**[Website](https://contextgeneric.dev/) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [CGP quickstart](https://contextgeneric.dev/docs/quickstart) · [Introduction post](https://contextgeneric.dev/blog/hypershell-release/)**

## What it does

Hypershell lets you define command pipelines, similar to shell scripts, directly in Rust's type
system. That arrangement brings four properties:

- **Type safety.** The compiler checks your shell-like programs.
- **Performance.** A DSL program is interpreted at compile time into native Rust code, with no runtime
  interpretation.
- **Extensibility.** Native Rust functions, such as HTTP requests or JSON parsing, compose with
  external CLI commands in the same pipeline.
- **Modularity.** The language itself is designed to be extended: you can add syntax and handlers
  without modifying the core library.

## Crate organization

Hypershell is organized into several crates, each with one purpose, so the language is easy to
extend.

- [`hypershell`](./crates/hypershell): the main crate, which re-exports the components a program
  needs and defines the default namespace and contexts for running Hypershell programs.
- [`hypershell-macro`](./crates/hypershell-macro): the `hypershell!` procedural macro, which provides
  the shell-like syntax.
- [`hypershell-components`](./crates/hypershell-components): the core abstract syntax, such as
  `SimpleExec` and `StreamingExec`, and the CGP components and traits the DSL builds on.
- [`hypershell-tokio-components`](./crates/hypershell-tokio-components): Tokio-based implementations
  for executing external CLI commands and handling file I/O.
- [`hypershell-reqwest-components`](./crates/hypershell-reqwest-components): native HTTP handlers,
  such as `SimpleHttpRequest` and `StreamingHttpRequest`, built on `reqwest`.
- [`hypershell-json-components`](./crates/hypershell-json-components): handlers for JSON
  serialization (`EncodeJson`) and deserialization (`DecodeJson`).
- [`hypershell-hash-components`](./crates/hypershell-hash-components): an extension crate with native
  handlers for checksumming (`Checksum`) and hex encoding (`BytesToHex`).
- [`hypershell-tungstenite-components`](./crates/hypershell-tungstenite-components): an extension
  crate that adds native WebSocket support.
- [`hypershell-examples`](./crates/hypershell-examples): the examples below.

## Getting started

Add `hypershell` and `cgp` to your `Cargo.toml`:

```toml
[dependencies]
cgp         = { version = "0.8.0" }
hypershell  = { version = "0.1.0" }

# The examples also need an async runtime and an HTTP client:
tokio = { version = "1", features = ["full"] }
reqwest = "0.12"
anyhow = "1.0"
```

The workspace builds against the `cgp` repository's `main` branch through its `[patch.crates-io]`
section, because it tracks unreleased CGP code. To build against a local `cgp` checkout, switch those
entries to the commented-out paths into `../cgp`.

## Building

These are the commands CI runs:

```sh
cargo build --workspace --all-targets
cargo test --workspace
```

The workspace pins `nightly-2026-11-07` in `rust-toolchain.toml`, because `.cargo/config.toml` turns
on the next-generation trait solver with `-Z next-solver=globally`, so `cargo` selects that nightly.

## Examples

The [`hypershell-examples`](./crates/hypershell-examples/examples) crate holds runnable programs, one
per feature. Run one with `cargo run -p hypershell-examples --example <name>`.

- [`hello.rs`](./crates/hypershell-examples/examples/hello.rs): executes `echo hello world!` through
  `SimpleExec`.
- [`hello_name.rs`](./crates/hypershell-examples/examples/hello_name.rs): passes dynamic values to a
  shell command with variable parameters (`FieldArg`).
- [`http_checksum_cli.rs`](./crates/hypershell-examples/examples/http_checksum_cli.rs): fetches a URL
  with `curl` and pipes the output to `sha256sum` and `cut` through streaming execution.
- [`http_checksum_client.rs`](./crates/hypershell-examples/examples/http_checksum_client.rs): fetches
  the URL with Hypershell's native HTTP client, then pipes the response through the same commands.
- [`http_checksum_native.rs`](./crates/hypershell-examples/examples/http_checksum_native.rs): the same
  checksum, using the native HTTP client and an extended DSL that adds `Checksum` to the syntax.
- [`rust_playground.rs`](./crates/hypershell-examples/examples/rust_playground.rs): encodes and
  decodes JSON, sending a Rust snippet to the Rust Playground API and parsing the response.
- [`github_issues.rs`](./crates/hypershell-examples/examples/github_issues.rs): reads a JSON web API
  and processes the result.
- [`nix_manual.rs`](./crates/hypershell-examples/examples/nix_manual.rs): combines a native HTTP
  request with external CLI tools.
- [`bluesky.rs`](./crates/hypershell-examples/examples/bluesky.rs): connects to the Bluesky firehose
  through `nix-shell` and `websocat`, then filters the stream with `grep`.
- [`bluesky_websocket.rs`](./crates/hypershell-examples/examples/bluesky_websocket.rs): the same
  firehose, using native WebSocket handling added to the DSL.
- [`compare_and_branch.rs`](./crates/hypershell-examples/examples/compare_and_branch.rs) and
  [`parallel_compare.rs`](./crates/hypershell-examples/examples/parallel_compare.rs): branch and
  compare pipelines, using the compare namespace.

## Learn CGP

The website is the main documentation for CGP: the
[quickstart](https://contextgeneric.dev/docs/quickstart), the
[tutorials](https://contextgeneric.dev/docs/tutorials/hello), the
[concepts](https://contextgeneric.dev/docs/concepts/), and the
[construct reference](https://contextgeneric.dev/docs/reference/). The
[project page](https://contextgeneric.dev/docs/projects/hypershell/) explains this repository, and
the [CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) records it in depth for
contributors and agents.

## Disclaimer

Hypershell is an **experimental** proof of concept that shows what CGP can do. Its purpose is to
demonstrate how CGP builds highly modular DSLs in Rust, not to be a production-ready shell
replacement.

## License

Licensed under the MIT license.
