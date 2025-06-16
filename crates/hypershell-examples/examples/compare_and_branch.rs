#![recursion_limit = "512"]

use hypershell::prelude::*;
use hypershell_examples::dsl::{Compare, If};
use hypershell_examples::presets::HypershellComparePreset;
use hypershell_hash_components::dsl::{BytesToHex, Checksum};
use reqwest::Client;
use sha2::Sha256;

pub type GetChecksumOf<Url> = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        Url,
        WithHeaders[ ],
    >
    | Checksum<Sha256>
    | BytesToHex
};

pub type Program = hypershell! {
    If<
        Compare<
            GetChecksumOf<FieldArg<"url_a">>,
            GetChecksumOf<FieldArg<"url_b">>,
        >,
        Pipe[
            SimpleExec<StaticArg<"echo">, WithArgs[StaticArg<"equals">]>,
            StreamToStdout,
        ],
        Pipe[
            SimpleExec<StaticArg<"echo">, WithArgs[StaticArg<"not equals">]>,
            StreamToStdout,
        ],
    >
};

#[cgp_context(MyAppComponents: HypershellComparePreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub url_a: String,
    pub url_b: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        url_a: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
        url_b: "https://nixos.org/manual/nixpkgs/unstable".to_owned(),
    };

    // app.handle(
    //     PhantomData::<Program>,
    //     ((<Vec<u8>>::new(), <Vec<u8>>::new()), Vec::new()),
    // )
    // .await?;

    Ok(())
}
