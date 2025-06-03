use hypershell::prelude::*;
use hypershell::presets::HypershellAppPreset;
use hypershell_macro::hypershell;
use reqwest::Client;

#[tokio::test]
async fn test_streaming_http_request() -> Result<(), Error> {
    pub type Program = hypershell! {
        BytesToStream
        |   StreamingHttpRequest<
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

    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub http_client: Client,
        pub keyword: String,
    }

    let app = TestApp {
        http_client: Client::new(),
        keyword: "Nix".to_owned(),
    };

    app.handle(PhantomData::<Program>, <Vec<u8>>::new()).await?;

    Ok(())
}
