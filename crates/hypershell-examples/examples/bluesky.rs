// This example demonstrates how to use Hypershell to stream data from a
// WebSocket and process it with CLI tools.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `StreamingExec` handler is used to run `nix-shell` to provision
//    the `websocat` utility. `websocat` then connects to the Bluesky
//    WebSocket feed at `wss://jetstream1.us-west.bsky.network/subscribe`.
//
// 2. The streaming output from `websocat` is piped to a second `StreamingExec`
//    handler, which runs `grep`. The `grep` command filters the stream
//    for lines containing a `keyword`.
//
// 3. The `keyword` is a dynamic argument provided by the `keyword` field
//    of the `MyApp` context, using `FieldArg<"keyword">`.
//
// 4. The final filtered output is piped to `StreamToStdout` to be printed
//    on the console.
//
// The `MyApp` struct defines the context for the program, providing the
// `keyword` for `grep`. It inherits from `HypershellPreset` to get the
// necessary components for running CLI commands.
//
// The `main` function initializes the `MyApp` context with a keyword and
// runs the program.

#![recursion_limit = "256"]

use hypershell::prelude::*;
use hypershell::presets::HypershellPreset;

pub type Program = hypershell! {
        StreamingExec<
            StaticArg<"nix-shell">,
            WithStaticArgs [
                "-p",
                "websocat",
                "--run",
                "websocat -nU wss://jetstream1.us-west.bsky.network/subscribe",
            ],
        >
    |   StreamingExec<
            StaticArg<"grep">,
            WithArgs [ FieldArg<"keyword"> ],
        >
    |   StreamToStdout
};

#[cgp_inherit(HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub keyword: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        keyword: "love".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
