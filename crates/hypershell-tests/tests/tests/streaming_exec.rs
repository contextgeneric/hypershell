use hypershell::prelude::*;
use hypershell::presets::HypershellAppPreset;

#[tokio::test]
async fn test_basic_streaming_exec() -> Result<(), Error> {
    pub type Program = hypershell! {
            StreamingExec<
                StaticArg<"nix-shell">,
                WithStaticArgs [
                    "-p",
                    "websocat",
                    "--run",
                    "websocat -nU wss://jetstream1.us-west.bsky.network/subscribe",
                ],
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

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
