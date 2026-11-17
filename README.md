[![Crates.io](https://img.shields.io/crates/v/hypershell.svg)](https://crates.io/crates/hypershell)
[![Docs.rs](https://docs.rs/hypershell/badge.svg)](https://docs.rs/hypershell)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/contextgeneric/hypershell/blob/main/LICENSE)
[![Tests](https://github.com/contextgeneric/hypershell/actions/workflows/tests.yml/badge.svg)](https://github.com/contextgeneric/hypershell/actions/workflows/tests.yml)

# Hypershell

Hypershell is a small language for writing shell-script-like programs in Rust, built with
[Context-Generic Programming (CGP)](https://contextgeneric.dev/), a language extension for Rust
with pluggable trait implementations at compile-time. A Hypershell program is a Rust *type*, and the
context that runs it is its interpreter: a type whose wiring chooses how each piece of the program
behaves. So both the language's syntax and its meaning can be extended without editing the crates
that define them.

Hypershell is an **experimental** proof of concept. It shows how CGP builds a modular language in
Rust, and it is not meant to replace a shell.

**[Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Examples](https://contextgeneric.dev/docs/projects/hypershell/examples/) · [CGP quickstart](https://contextgeneric.dev/docs/quickstart) · [Introduction post](https://contextgeneric.dev/blog/hypershell-release/)**

## A program that is a type

This is the smallest Hypershell program. It runs `echo hello world!` and prints the output:

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

`Program` is a type alias, not a value. Each piece of it, such as `SimpleExec` or `StreamToStdout`,
is an empty struct that says nothing about how it runs, and the `hypershell!` macro only rewrites
the shell-like surface into that type: the `|` becomes a pipeline, the brackets a type-level list,
and each string a type-level string.

Running the program is one `handle` call on a context. `HypershellCli` is the context here, and this
is its entire definition:

```rust
pub struct HypershellCli;

delegate_components! {
    HypershellCli {
        namespace HypershellNamespace;
    }
}
```

The one line joins `HypershellNamespace`, which routes each piece of syntax to the implementation,
or **provider**, that interprets it. The compiler resolves every choice while it type-checks the
program, so the call compiles to direct calls to those providers, with no interpreter running at run
time. A context of your own can join the same namespace, add fields that a program reads, such as a
URL, or join a namespace that extends the language with new syntax.

## Installation

Add `hypershell`, `cgp`, and an async runtime to your `Cargo.toml`:

```toml
[dependencies]
cgp = "0.8.0"
hypershell = "0.2.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The `hypershell!` macro expects `hypershell::prelude::*` in scope, and its expansion names the `cgp`
crate, so a program's crate depends on both.

The library crates build on stable Rust, and so does a small program such as the one above. Large
programs can need nightly Rust with the next-generation trait solver. On stable Rust 1.98.1, the
repository's two compare examples, which nest whole programs inside others, grew `rustc` to about
7 GB while type-checking and were stopped; on nightly with `-Z next-solver=globally`, a fresh check
of one of them took about 9 seconds and 460 MB. A crate with programs of that size can copy the
repository's `rust-toolchain.toml` and `.cargo/config.toml`, which select a dated nightly and turn
the solver on.

## The crates

Hypershell is split into crates by backend, so a program compiles only the libraries it uses:

- [`hypershell`](./crates/hypershell): the main crate, with `HypershellNamespace`, the error wiring,
  the `HypershellCli` and `HypershellHttp` contexts, and the prelude a program imports.
- [`hypershell-macro`](./crates/hypershell-macro): the `hypershell!` macro, which provides the
  shell-like syntax.
- [`hypershell-components`](./crates/hypershell-components): the syntax, such as `SimpleExec` and
  `StreamingExec`, and the CGP components and traits the language builds on.
- [`hypershell-tokio-components`](./crates/hypershell-tokio-components): running external commands,
  files, and streams, on Tokio.
- [`hypershell-reqwest-components`](./crates/hypershell-reqwest-components): HTTP requests, such as
  `SimpleHttpRequest` and `StreamingHttpRequest`, on `reqwest`.
- [`hypershell-json-components`](./crates/hypershell-json-components): JSON encoding with
  `EncodeJson` and decoding with `DecodeJson`.
- [`hypershell-hash-components`](./crates/hypershell-hash-components): an extension with native
  checksums (`Checksum`) and hex encoding (`BytesToHex`).
- [`hypershell-tungstenite-components`](./crates/hypershell-tungstenite-components): an extension
  with native WebSockets.

The two extensions are not part of `HypershellNamespace`, so a program that uses them joins an
extended namespace or wires them on its context, as the examples show. The unpublished
[`hypershell-examples`](./crates/hypershell-examples) crate holds the examples below.

## Examples

Each example is a runnable program, run from the repository's root with
`cargo run -p hypershell-examples --example <name>`. Most need the network and a few common
command-line tools.

- [`hello.rs`](./crates/hypershell-examples/examples/hello.rs): runs `echo hello world!` through
  `SimpleExec`.
- [`hello_name.rs`](./crates/hypershell-examples/examples/hello_name.rs): passes a value from a field
  of the context to a shell command with `FieldArg`.
- [`http_checksum_cli.rs`](./crates/hypershell-examples/examples/http_checksum_cli.rs): fetches a URL
  with `curl` and pipes the output to `sha256sum` and `cut` as streaming commands.
- [`http_checksum_client.rs`](./crates/hypershell-examples/examples/http_checksum_client.rs): fetches
  the URL with the native HTTP client, then pipes the response through the same commands.
- [`http_checksum_native.rs`](./crates/hypershell-examples/examples/http_checksum_native.rs): the same
  checksum in native stages, from a language extended with `Checksum`.
- [`nix_manual.rs`](./crates/hypershell-examples/examples/nix_manual.rs): combines a native HTTP
  request with external command-line tools.
- [`save_webpage.rs`](./crates/hypershell-examples/examples/save_webpage.rs): downloads a webpage as a
  stream and writes it to a file.
- [`github_issues.rs`](./crates/hypershell-examples/examples/github_issues.rs): calls a JSON web API
  and decodes the result into a Rust type.
- [`rust_playground.rs`](./crates/hypershell-examples/examples/rust_playground.rs): sends a Rust
  snippet to the Rust Playground API as JSON and decodes the response.
- [`bluesky.rs`](./crates/hypershell-examples/examples/bluesky.rs): connects to the Bluesky firehose
  through `nix-shell` and `websocat`, then filters the stream with `grep`.
- [`bluesky_websocket.rs`](./crates/hypershell-examples/examples/bluesky_websocket.rs): the same
  firehose, with native WebSockets added to the language.
- [`compare_and_branch.rs`](./crates/hypershell-examples/examples/compare_and_branch.rs) and
  [`parallel_compare.rs`](./crates/hypershell-examples/examples/parallel_compare.rs): compare the
  outputs of pipelines and branch on the result, with syntax the examples crate adds.

## Status and costs

Hypershell is a demonstration, and these are the costs to weigh first:

- **Large programs need nightly.** As [Installation](#installation) describes, the library and small
  programs build on stable Rust, and programs that nest others deeply can need nightly with the new
  trait solver.
- **Programs are fixed at compile time.** A program is a type, so it cannot be loaded or changed
  while the application runs. Values a program needs at run time, such as a URL, come from fields of
  its context.
- **Error messages are long.** A mistake in a program is a compile error, and the compiler's raw
  message lists every step it took before it failed. Checking the program with `check_components!`
  and reading the error with
  [`cargo cgp check`](https://contextgeneric.dev/docs/tooling/cargo-cgp/check) helps:
  `cargo cgp check` leads with the root cause for the classes it recognizes, and the tool does not
  yet reshape every class.
- **It is lightly tested.** Its value is as a worked example rather than a tool for production, and
  the [limitations page](https://contextgeneric.dev/docs/projects/hypershell/limitations) describes
  where the design stops and when a shell script or plain Rust fits better.

## Building

These are the commands CI runs:

```sh
cargo build --workspace --all-targets
cargo test --workspace
```

The workspace pins `nightly-2026-11-16` in `rust-toolchain.toml`, because `.cargo/config.toml` turns
on the next-generation trait solver with `-Z next-solver=globally`, so `cargo` selects that nightly
in the repository. Contributors and agents start from [AGENTS.md](AGENTS.md), which names these
checks.

## Learn more

The [project page](https://contextgeneric.dev/docs/projects/hypershell/) explains Hypershell: its
examples as short tutorials, the design one idea at a time, and guides to writing and debugging a
program. For CGP itself, start with the [quickstart](https://contextgeneric.dev/docs/quickstart),
then the [tutorials](https://contextgeneric.dev/docs/tutorials/hello) and the
[concepts](https://contextgeneric.dev/docs/concepts/), and the
[type-level DSLs](https://contextgeneric.dev/docs/concepts/type-level-dsls) page explains the
technique behind Hypershell with a smaller language. The
[CGP knowledge base](https://github.com/contextgeneric/cgp-knowledge-base) records the project in
depth for contributors and agents.
