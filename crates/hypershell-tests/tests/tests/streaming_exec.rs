use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use futures::io::{AsyncReadExt, copy};
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    BytesToStream, FieldArg, Pipe, StaticArg, StreamingExec, WithArgs, WithStaticArgs,
};
use tokio_util::compat::TokioAsyncWriteCompatExt;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = Pipe<
        Product![
            BytesToStream,
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
            >
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

    let output = app.handle(PhantomData::<Program>, Vec::new()).await?;

    let output = output.take(102400);

    let mut stdout = tokio::io::stdout().compat_write();

    copy(output, &mut stdout).await?;

    Ok(())
}
