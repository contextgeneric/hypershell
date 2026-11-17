[![Crates.io](https://img.shields.io/crates/v/hypershell-tungstenite-components.svg)](https://crates.io/crates/hypershell-tungstenite-components)
[![Docs.rs](https://docs.rs/hypershell-tungstenite-components/badge.svg)](https://docs.rs/hypershell-tungstenite-components)

# hypershell-tungstenite-components: WebSockets as a Hypershell extension

`hypershell-tungstenite-components` interprets the `WebSocket` syntax of [Hypershell](https://docs.rs/hypershell), a small language for shell-script-like programs in Rust built with [Context-Generic Programming (CGP)](https://contextgeneric.dev/), on
[`tokio-tungstenite`](https://docs.rs/tokio-tungstenite). `HandleWebsocket` connects to the URL a
program names, sends the stage's input to the server as binary messages, and passes the replies on
to the next stage as a stream, each text message ending in a newline. The
`HypershellTungsteniteProvider` bundle collects its wiring.

**[hypershell](https://docs.rs/hypershell) · [Project page](https://contextgeneric.dev/docs/projects/hypershell/) · [Repository](https://github.com/contextgeneric/hypershell)**

`HypershellNamespace` does not route `WebSocket`, so a program that uses it runs on a context that
wires the bundle. The repository's
[`bluesky_websocket`](https://contextgeneric.dev/docs/projects/hypershell/examples/bluesky-websocket) example wires it onto a single context.
