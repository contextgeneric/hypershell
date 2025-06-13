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
