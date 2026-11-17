[![Crates.io](https://img.shields.io/crates/v/hypershell-reqwest-components.svg)](https://crates.io/crates/hypershell-reqwest-components)
[![Docs.rs](https://docs.rs/hypershell-reqwest-components/badge.svg)](https://docs.rs/hypershell-reqwest-components)

# hypershell-reqwest-components: HTTP requests on reqwest

`hypershell-reqwest-components` interprets the HTTP syntax of
[Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust
built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/), on
[`reqwest`](https://docs.rs/reqwest). It sends simple and streaming requests with the method, URL,
and headers a program names, and reports a failed response as an error.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

Its items sit in three modules:

- **`dsl`:** the core request syntax `CoreHttpRequest`.
- **`components`:** `CanUpdateRequestBuilder`, which adds a program's headers to a request, and
  `HasReqwestClient`, which reads the HTTP client from the context.
- **`providers`:** the request providers, the method extractor and header updaters, `StreamToBody`
  for a streamed request body, and the `ErrorResponse` error, collected in the
  `HypershellReqwestProvider` bundle.

The crate builds on [`hypershell-tokio-components`](https://docs.rs/hypershell-tokio-components) for
its stream types. `HypershellNamespace` routes its syntax, and the `HypershellHttp` context supplies
the client.
