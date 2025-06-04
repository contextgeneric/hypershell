use hypershell::prelude::*;
use hypershell_macro::hypershell;
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

#[cgp_context(MyAppComponents: HypershellPreset)]
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
