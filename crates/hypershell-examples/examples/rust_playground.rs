use hypershell::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub type Program = hypershell! {
    EncodeJson
    |   SimpleHttpRequest<
            PostMethod,
            StaticArg<"https://play.rust-lang.org/meta/gist">,
            WithHeaders [
                Header<
                    StaticArg<"Content-Type">,
                    StaticArg<"application/json">,
                >
            ],
        >
    |   DecodeJson<Response>
};

#[derive(Serialize)]
pub struct Request {
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub id: String,
    pub url: String,
    pub code: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = HypershellHttp {
        http_client: Client::new(),
    };

    let input = Request {
        code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
    };

    let output = app.handle(PhantomData::<Program>, input).await?;

    println!("Created new Rust playground gist with response: {output:#?}");

    Ok(())
}
