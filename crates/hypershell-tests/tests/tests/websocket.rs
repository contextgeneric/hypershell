use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    FieldArg, Pipe, StaticArg, StreamToStdout, StreamingExec, WebSocket, WithArgs,
};
use hypershell_macro::hypershell;
use hypershell_tokio_components::types::TokioAsyncReadStream;
use tokio::io::simplex;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = hypershell! {
            WebSocket<
                StaticArg<"wss://jetstream1.us-west.bsky.network/subscribe">,
                (),
            >
        |   StreamingExec<
                StaticArg<"grep">,
                WithArgs [ FieldArg<"keyword"> ],
            >
        |   StreamToStdout
    };

    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub keyword: String,
    }

    let app = TestApp {
        keyword: "love".to_owned(),
    };

    let (read, _write) = simplex(102400);
    let input = TokioAsyncReadStream::from(read);

    app.handle(PhantomData::<Program>, input).await?;

    Ok(())
}
