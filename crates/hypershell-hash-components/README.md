[![Crates.io](https://img.shields.io/crates/v/hypershell-hash-components.svg)](https://crates.io/crates/hypershell-hash-components)
[![Docs.rs](https://docs.rs/hypershell-hash-components/badge.svg)](https://docs.rs/hypershell-hash-components)

# hypershell-hash-components: checksums as a Hypershell extension

`hypershell-hash-components` extends [Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/) with two pieces of syntax and their providers, in a
crate the core language does not depend on. `Checksum<Hasher>` hashes a stream of bytes with any
`Digest` hasher, such as `sha2::Sha256`, through `HandleStreamChecksum`, and `BytesToHex` writes
bytes as hexadecimal, through `HandleBytesToHex`.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

`HypershellNamespace` does not route this syntax, so a program that uses it runs on a context that
joins an extended namespace or wires the providers itself. The repository's
[`http_checksum_native`](https://contextgeneric.dev/docs/projects/hypershell/examples/http-checksum-native) example joins a namespace that inherits
the base language and adds `Checksum`, changing one line of its context.
