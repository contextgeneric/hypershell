// This example demonstrates a pipeline that combines a native HTTP request
// with subsequent processing using external CLI tools.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `StreamingHttpRequest` handler fetches the content of the Nixpkgs manual.
//    The URL is hardcoded as a `StaticArg`.
//
// 2. The downloaded content is streamed to a `StreamingExec` handler that runs
//    `tr "[:lower:]" "[:upper:]"` to convert the text to uppercase.
//
// 3. The transformed text is then streamed to another `StreamingExec` handler
//    that runs `grep`. `grep` filters the stream for lines containing a
//    `keyword`, performing a case-insensitive search (`-i`).
//
// 4. The `keyword` is provided dynamically by the `keyword` field of the
//    `MyApp` context.
//
// 5. The final filtered output is piped to `StreamToStdout`.
//
// The `MyApp` context provides the `http_client` for the HTTP request and the
// `keyword` for the `grep` command.
//
// The `main` function initializes the `MyApp` context and runs the program.

use hypershell::prelude::*;
use hypershell::presets::HypershellPreset;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingHttpRequest<
            GetMethod,
            StaticArg<"https://nixos.org/manual/nixpkgs/unstable/">,
            WithHeaders<Nil>,
        >
    |   StreamingExec<
            StaticArg<"tr">,
            WithStaticArgs [
                "[:lower:]",
                "[:upper:]",
            ],
        >
    |   StreamingExec<
            StaticArg<"grep">,
            WithArgs [
                StaticArg<"-i">,
                FieldArg<"keyword">,
            ],
        >
    | StreamToStdout
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub keyword: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        keyword: "Nix".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
