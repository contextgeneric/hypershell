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

### Example: Native HTTP and CLI Pipeline

Here's an example that showcases Hypershell's ability to mix native Rust handlers with CLI commands. This program fetches a webpage using the native `reqwest` HTTP client, then pipes the response to the `sha256sum` and `cut` CLI commands to extract the checksum.

```rust
use cgp::prelude::*;
use hypershell::prelude::*;
use reqwest::Client;
use std::marker::PhantomData;
use anyhow::Error;

// Define the Hypershell program as a type
pub type Program = hypershell! {
    // Use a native HTTP client for the request
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[],
    >
    // Pipe the streaming response to `sha256sum`
    |   StreamingExec<
            StaticArg<"sha256sum">,
            WithStaticArgs[],
        >
    // Pipe the output of `sha256sum` to `cut`
    |   StreamingExec<
            StaticArg<"cut">,
            WithStaticArgs [
                "-d",
                " ",
                "-f",
                "1",
            ],
        >
    // Stream the final result to STDOUT
    |   StreamToStdout
};

// Define a custom context to provide the `url` field and an HTTP client.
// The context inherits all standard Hypershell functionality from `HypershellPreset`.
#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Create an instance of our custom context
    let app = MyApp {
        http_client: Client::new(),
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
    };

    // Run the program with the context.
    // The initial input is an empty Vec, as the first handler generates the data.
    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
```

When you run this program, it will print the SHA256 checksum of the Nixpkgs manual page.

## Disclaimer

Hypershell is an **experimental** proof of concept designed to showcase the capabilities of Context-Generic Programming (CGP). Its primary purpose is to demonstrate how CGP can be used to build highly modular DSLs in Rust, rather than to be a production-ready shell replacement.
