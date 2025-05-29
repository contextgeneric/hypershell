use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use futures::io::copy;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    FieldArg, GetMethod, Pipe, StaticArg, StreamingExec, StreamingHttpRequest, WithArgs,
    WithHeaders,
};
use reqwest::Client;
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_streaming_http_request() -> Result<(), Error> {
    pub type Program = Pipe<
        Product![
            StreamingHttpRequest<
                GetMethod,
                StaticArg<symbol!("https://nixos.org/manual/nixpkgs/unstable/")>,
                WithHeaders<Nil>,
            >,
            StreamingExec<
                StaticArg<symbol!("grep")>,
                WithArgs<Product![FieldArg<symbol!("keyword")>]>,
            >
        ],
    >;

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

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    let mut stdout = tokio::io::stdout().compat_write();

    copy(output, &mut stdout).await?;

    Ok(())
}
