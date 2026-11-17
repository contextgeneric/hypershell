[![Crates.io](https://img.shields.io/crates/v/hypershell-json-components.svg)](https://crates.io/crates/hypershell-json-components)
[![Docs.rs](https://docs.rs/hypershell-json-components/badge.svg)](https://docs.rs/hypershell-json-components)

# hypershell-json-components: JSON stages for Hypershell

`hypershell-json-components` interprets the JSON syntax of [Hypershell](https://docs.rs/hypershell),
a small language for shell-script-like programs in Rust built with
[Context-Generic Programming (CGP)](https://contextgeneric.dev/), on
[`serde_json`](https://docs.rs/serde_json). `HandleEncodeJson` interprets `EncodeJson`, which writes
a Rust value as JSON bytes, and `HandleDecodeJson` interprets `DecodeJson<T>`, which reads JSON
bytes into a `T`. Both are collected in the `HypershellJsonProvider` bundle.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

The crate is `no_std` and links `alloc`, and `HypershellNamespace` routes both stages for every
context that joins it.
