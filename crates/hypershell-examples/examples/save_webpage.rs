// This example demonstrates how to download a webpage and save it to a file.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. A `StreamingHttpRequest` handler fetches content from a given `url`.
//    The URL is provided dynamically from the `url` field of the `MyApp` context.
//
// 2. The response stream is piped to the `WriteFile` handler.
//
// 3. The `WriteFile` handler saves the incoming stream to a file at a given
//    path. The path is provided dynamically from the `file_path` field of the
//    `MyApp` context.
//
// The `MyApp` context provides the `http_client` for the request, the `url` to
// download, and the `file_path` where the content should be saved.
//
// The `main` function initializes the `MyApp` context with these values and
// runs the program. A confirmation message is printed to the console upon
// completion.

use hypershell::prelude::*;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        FieldArg<"url">,
        WithHeaders[ ],
    >
    |   WriteFile<FieldArg<"file_path">>
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url: String,
    pub file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
        file_path: "nix_manual.html".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("Webpage saved to nix_manual.html");

    Ok(())
}
