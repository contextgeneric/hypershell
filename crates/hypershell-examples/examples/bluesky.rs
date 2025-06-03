#![recursion_limit = "256"]

use hypershell::prelude::*;
use hypershell::presets::HypershellPreset;

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

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub keyword: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        keyword: "love".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
