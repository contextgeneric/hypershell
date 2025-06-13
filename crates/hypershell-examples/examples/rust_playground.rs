// This example demonstrates how to work with native Rust types and JSON
// serialization/deserialization in a Hypershell pipeline. It creates a
// new Gist on the Rust Playground.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. The `EncodeJson` handler takes a native Rust struct (`Request`) that
//    implements `serde::Serialize` as input and serializes it into a
//    JSON byte stream.
//
// 2. The JSON stream is piped to a `SimpleHttpRequest` handler, which sends
//    a POST request to the Rust Playground's Gist API. The `Content-Type`
//    header is set to `application/json`.
//
// 3. The JSON response from the API is piped to the `DecodeJson<Response>`
//    handler, which deserializes the JSON stream back into a native Rust
//    struct (`Response`) that implements `serde::Deserialize`.
//
// The program is executed using `HypershellHttp`, a predefined context that
// includes an HTTP client.
//
// In the `main` function, an instance of the `Request` struct is created
// and passed as the input to `app.handle`. The `handle` call returns the
// deserialized `Response` struct as its output.

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
