use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    BytesToStream, FieldArg, Pipe, StaticArg, StreamToStdout, StreamingExec, WebSocket, WithArgs,
};
use hypershell_macro::hypershell;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = hypershell! {
            BytesToStream
        |   WebSocket<
                StaticArg<"wss://jetstream1.us-west.bsky.network/subscribe">,
                (),
            >
            // StreamingExec<
            //     StaticArg<"nix-shell">,
            //     WithStaticArgs [
            //         "-p",
            //         "websocat",
            //         "--run",
            //         "websocat -nU wss://jetstream1.us-west.bsky.network/subscribe",
            //     ],
            // >
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

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
