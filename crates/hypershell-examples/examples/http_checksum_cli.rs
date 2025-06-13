// This example demonstrates how to create a streaming pipeline using only
// external CLI commands.
//
// The Hypershell program is defined as a `Program` type. It is equivalent to
// the following shell command:
//
// curl $url | sha256sum | cut -d ' ' -f 1
//
// The pipeline consists of the following steps:
//
// 1. A `StreamingExec` handler runs `curl` to fetch the content from a URL.
//    The URL is provided dynamically from the `url` field of the `MyApp`
//    context.
//
// 2. The output of `curl` is streamed to a second `StreamingExec` handler,
//    which runs `sha256sum` to compute the SHA256 checksum of the stream.
//
// 3. The output of `sha256sum` (e.g., "checksum filename") is streamed to a
//    third `StreamingExec` handler, which runs `cut` to extract only the
//    checksum part.
//
// 4. The final result is piped to `StreamToStdout` to be printed to the
//    console.
//
// The `MyApp` context provides the `url` field required by the `curl` command.
//
// The `main` function initializes `MyApp` with a URL and executes the program.

use hypershell::prelude::*;

pub type Program = hypershell! {
    StreamingExec<
        StaticArg<"curl">,
        WithArgs [
            FieldArg<"url">,
        ],
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

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;
    Ok(())
}
