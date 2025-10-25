// This example demonstrates how to mix native handlers with CLI commands in a
// streaming pipeline. It improves upon the `http_checksum_cli` example by
// replacing the `curl` command with a native HTTP client.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A native `StreamingHttpRequest` handler sends a GET request to a URL.
//    This handler is part of Hypershell's standard library and uses the
//    `reqwest` crate behind the scenes.
//
// 2. The URL for the request is provided dynamically from the `url` field of
//    the `MyApp` context.
//
// 3. The response body is streamed to a `StreamingExec` handler that runs
//    `sha256sum` to compute the checksum.
//
// 4. The output is then piped to another `StreamingExec` handler that runs
//    `cut` to extract the checksum value.
//
// 5. The final result is piped to `StreamToStdout`.
//
// The `MyApp` context now provides both an `http_client` (a `reqwest::Client`
// instance) and the `url` string, which are required by the
// `StreamingHttpRequest` handler.
//
// The `main` function initializes the `MyApp` context and executes the program.

use hypershell::prelude::*;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[ ],
    >
    |   StreamingExec<
            StaticArg<"sha256sum">,
            WithStaticArgs [],
        >
    |   StreamingExec<
            StaticArg<"cut">,
            WithStaticArgs [
                "-d",
                " ",
                "-f",
                "1",
            ],
        >
    | StreamToStdout
};

#[cgp_inherit(HypershellPreset)]
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
