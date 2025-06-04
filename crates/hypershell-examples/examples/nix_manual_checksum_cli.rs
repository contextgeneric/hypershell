use hypershell::prelude::*;
use hypershell_macro::hypershell;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingExec<
        StaticArg<"curl">,
        WithStaticArgs [
            "https://nixos.org/manual/nixpkgs/unstable/",
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = HypershellHttp {
        http_client: Client::new(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
