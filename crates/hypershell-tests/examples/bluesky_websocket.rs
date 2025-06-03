use hypershell::prelude::*;
use hypershell::presets::HypershellAppPreset;
use hypershell_tokio_components::types::TokioAsyncReadStream;
use tokio::io::simplex;

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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = TestApp {
        keyword: "love".to_owned(),
    };

    let (read, _write) = simplex(102400);
    let input = TokioAsyncReadStream::from(read);

    app.handle(PhantomData::<Program>, input).await?;

    Ok(())
}
