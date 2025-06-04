use hypershell::prelude::*;
use hypershell_macro::hypershell;
use reqwest::Client;

pub type Program = hypershell! {
    StreamingHttpRequest<
        GetMethod,
        StaticArg<"https://nixos.org/manual/nixpkgs/unstable/">,
        WithHeaders<Nil>,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = HypershellHttp {
        http_client: Client::new(),
    };

    app.handle(PhantomData::<Program>, Vec::new()).await?;

    Ok(())
}
