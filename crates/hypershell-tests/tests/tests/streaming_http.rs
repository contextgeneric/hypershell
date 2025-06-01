use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    FieldArg, GetMethod, Pipe, StaticArg, StreamToStdout, StreamingExec, StreamingHttpRequest,
    WithArgs, WithHeaders, WithStaticArgs,
};
use hypershell_macro::hypershell;
use reqwest::Client;

#[tokio::test]
async fn test_streaming_http_request() -> Result<(), Error> {
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

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
