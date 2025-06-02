use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    FieldArg, Pipe, StaticArg, StreamToStdout, StreamingExec, WithArgs, WithStaticArgs,
};

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = Pipe<
        Product![
            StreamingExec<
                StaticArg<symbol!("nix-shell")>,
                WithStaticArgs<
                    Product![
                        symbol!("-p"),
                        symbol!("websocat"),
                        symbol!("--run"),
                        symbol!("websocat -nU wss://jetstream1.us-west.bsky.network/subscribe"),
                    ],
                >,
            >,
            StreamingExec<
                StaticArg<symbol!("grep")>,
                WithArgs<Product![FieldArg<symbol!("keyword")>]>,
            >,
            StreamToStdout,
        ],
    >;

    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub keyword: String,
    }

    let app = TestApp {
        keyword: "love".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
