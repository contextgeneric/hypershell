// This example demonstrates a fully native streaming pipeline by extending
// Hypershell with custom handlers for checksum calculation and hex encoding.
// It builds upon the `http_checksum_client` example by replacing the
// `sha256sum` and `cut` commands with native implementations.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `StreamingHttpRequest` handler fetches content from a URL, similar to
//    the previous example.
//
// 2. The response stream is piped to a custom `Checksum<Sha256>` handler. This
//    handler is defined in the `hypershell-hash-components` crate and computes
//    the SHA256 checksum natively using the `sha2` crate.
//
// 3. The raw byte output of the `Checksum` handler is piped to another custom
//    handler, `BytesToHex`, which encodes the bytes into a hexadecimal string.
//
// 4. The final hex string is piped to `StreamToStdout`.
//
// Since `Checksum` and `BytesToHex` are not part of the default preset, a
// custom `MyAppPreset` is defined to extend `HypershellPreset`. This new
// preset overrides the `HandlerComponent` to include the providers for the
// new handlers (`HandleStreamChecksum` and `HandleBytesToHex`).
//
// The `MyApp` context is configured to use `MyAppPreset`, enabling it to
// execute the program with the new native handlers.
//
// The `main` function initializes the `MyApp` context and executes the program.

#![recursion_limit = "256"]

use hypershell::prelude::*;
use hypershell_examples::presets::HypershellChecksumPreset;
use hypershell_hash_components::dsl::{BytesToHex, Checksum};
use reqwest::Client;
use sha2::Sha256;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[ ],
    >
    | Checksum<Sha256>
    | BytesToHex
    | StreamToStdout
};

#[cgp_inherit(HypershellChecksumPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
