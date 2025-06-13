# Hypershell

Hypershell is a modular, *type-level* domain-specific language (DSL) for writing shell-script-like programs in Rust. It is powered by [context-generic programming (CGP)](https://contextgeneric.dev/), which makes it possible for users to *extend* or *modify* both the language syntax and semantics.

## Learn More

This README provides a brief overview of Hypershell. To learn about the concepts behind Hypershell, Context-Generic Programming, and how to create your own extensions, please read the introductory blog post:

[**Hypershell: A Type-Level DSL for Shell-Scripting in Rust**](https://contextgeneric.dev/blog/hypershell-release/)

## What is Hypershell?

Hypershell allows you to define complex command pipelines, similar to shell scripts, but directly within Rust's type system. This approach provides several key benefits:

-   **Type Safety**: Your shell-like programs are checked by the Rust compiler.
-   **Performance**: DSL programs are interpreted at compile-time into native Rust code with no runtime overhead.
-   **Extensibility**: Seamlessly integrate native Rust functions (like HTTP requests or JSON parsing) with external CLI commands in your pipelines.
-   **Modularity**: The language itself is designed to be extended. You can add new syntax and handlers without modifying the core library.

## Key Features

-   **Type-Level DSL**: Define shell scripts as Rust types.
-   **Extensible Syntax and Semantics**: Powered by CGP, allowing for deep customization.
-   **Mix CLI and Native Handlers**: Combine external commands like `sha256sum` or `cut` with native Rust logic for HTTP requests, JSON processing, and more.
-   **Streaming Pipelines**: Efficiently stream I/O between handlers, just like in a traditional shell.
-   **Compile-Time Interpretation**: Your DSL programs are resolved at compile-time, resulting in highly performant native code.

## Crate Organization

Hypershell is organized into several crates, each with a specific purpose. This modular structure allows for clear separation of concerns and makes the language easier to extend.

-   [`hypershell`](./crates/hypershell): The main crate that re-exports all the necessary components for using the Hypershell DSL. It defines the default presets and contexts for running Hypershell programs.
-   [`hypershell-macro`](./crates/hypershell-macro): Contains the `hypershell!` procedural macro, which provides the shell-like syntax for the DSL.
-   [`hypershell-components`](./crates/hypershell-components): Defines the core abstract syntax (e.g., `SimpleExec`, `StreamingExec`) and the fundamental CGP components and traits that form the basis of the DSL.
-   [`hypershell-tokio-components`](./crates/hypershell-tokio-components): Provides Tokio-based implementations for executing external CLI commands and handling file I/O.
-   [`hypershell-reqwest-components`](./crates/hypershell-reqwest-components): Implements native HTTP handlers (e.g., `SimpleHttpRequest`, `StreamingHttpRequest`) using the `reqwest` crate.
-   [`hypershell-json-components`](./crates/hypershell-json-components): Provides handlers for JSON serialization (`EncodeJson`) and deserialization (`DecodeJson`).
-   [`hypershell-hash-components`](./crates/hypershell-hash-components): An extension crate that offers native handlers for checksumming (`Checksum`) and hex encoding (`BytesToHex`).
-   [`hypershell-tungstenite-components`](./crates/hypershell-tungstenite-components): An extension crate that adds native WebSocket support to Hypershell.
-   [`hypershell-examples`](./crates/hypershell-examples): Contains a collection of examples demonstrating various features and use cases of Hypershell.

## Getting Started

### Installation

Add `hypershell` and `cgp` to your `Cargo.toml`:

```toml
[dependencies]
cgp         = { version = "0.4.1" }
hypershell  = { version = "0.1.0" }

# You'll also need tokio for async runtime and reqwest for the example
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
anyhow = "1.0"
```

### Examples

The `hypershell-examples` crate contains various examples demonstrating different features and use cases of Hypershell. You can find the full source code for these examples in the [`crates/hypershell-examples/examples/`](./crates/hypershell-examples/examples) directory.

Here are a few hand-picked examples with short descriptions:

*   [`hello.rs`](./crates/hypershell-examples/examples/hello.rs): A basic "hello world" program that executes `echo hello world!` using `SimpleExec`.
*   [`hello_name.rs`](./crates/hypershell-examples/examples/hello_name.rs): Demonstrates using variable parameters (`FieldArg`) to pass dynamic values to shell commands.
*   [`http_checksum_cli.rs`](./crates/hypershell-examples/examples/http_checksum_cli.rs): Fetches a URL using `curl` and pipes the output to `sha256sum` and `cut` via streaming execution.
*   [`http_checksum_client.rs`](./crates/hypershell-examples/examples/http_checksum_client.rs): Fetches a URL using Hypershell's native HTTP client, then pipes the response to `sha256sum` and `cut` via streaming execution.
*   [`http_checksum_native.rs`](./crates/hypershell-examples/examples/http_checksum_native.rs): The same checksum functionality, but uses Hypershell's native HTTP client together with an extended version of the DSL that introduces `Checksum` to the language syntax, showcasing the extensibility of the DSL.
*   [`rust_playground.rs`](./crates/hypershell-examples/examples/rust_playground.rs): Shows how to encode and decode JSON, sending a Rust code snippet to the Rust Playground API and parsing its response.
*   [`bluesky.rs`](./crates/hypershell-examples/examples/bluesky.rs): Connects to the Bluesky social media firehose via `nix-shell` and `websocat`, and then filters the stream using `grep`.
*   [`bluesky_websocket.rs`](./crates/hypershell-examples/examples/bluesky_websocket.rs): The same Bluesky firehose example, but extends the DSL with native Websocket handling and using it in the program, showcasing the extensibility of the DSL.

## Disclaimer

Hypershell is an **experimental** proof of concept designed to showcase the capabilities of Context-Generic Programming (CGP). Its primary purpose is to demonstrate how CGP can be used to build highly modular DSLs in Rust, rather than to be a production-ready shell replacement.
