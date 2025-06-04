use hypershell::prelude::*;
use hypershell_macro::hypershell;

pub type Program = hypershell! {
    StreamingExec<
        StaticArg<"curl">,
        WithArgs [
            FieldArg<"url">,
        ],
    >
    |   StreamingExec<
            StaticArg<"sha256sum">,
            WithStaticArgs [],
        >
    |   StreamingExec<
            StaticArg<"cut">,
            WithStaticArgs [
                "-d",
                " ",
                "-f",
                "1",
            ],
        >
    | StreamToStdout
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        url: "https://nixos.org/manual/nixpkgs/unstable/".to_owned(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;
    Ok(())
}
