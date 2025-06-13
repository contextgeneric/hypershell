// This example demonstrates how to use Hypershell to interact with a
// JSON-based web API.
//
// The Hypershell program is defined as a `Program` type. It performs the
// following steps:
//
// 1. Constructs a URL for the GitHub API to list issues for a repository.
//    The URL is built by joining static arguments with dynamic arguments
//    extracted from fields of the `MyApp` context, such as `base_url`,
//    `github_org`, and `github_repo`.
//
// 2. The dynamic arguments are URL-encoded to ensure the final URL is valid.
//
// 3. A `SimpleHttpRequest` is sent to the constructed URL using the GET method,
//    with a "User-Agent" header.
//
// 4. The JSON response from the API is piped to `DecodeJson`, which deserializes
//    the JSON byte stream into a `Vec<Issue>`.
//
// The `MyApp` struct defines the context for running the Hypershell program.
// It uses `#[cgp_context]` to inherit the necessary components from
// `HypershellPreset` for executing the program. It also uses `#[derive(HasField)]`
// to expose its fields to be used by `FieldArg` within the program definition.
//
// The `main` function sets up the `MyApp` context with the required values,
// executes the `Program` using `app.handle`, and prints the fetched issues.

use hypershell::prelude::*;
use hypershell::presets::HypershellPreset;
use reqwest::Client;
use serde::Deserialize;

pub type Program = hypershell! {
    SimpleHttpRequest<
        GetMethod,
        JoinArgs [
            FieldArg<"base_url">,
            StaticArg<"/repos/">,
            UrlEncodeArg<FieldArg<"github_org">>,
            StaticArg<"/">,
            UrlEncodeArg<FieldArg<"github_repo">>,
            StaticArg<"/issues">,
        ],
        WithHeaders [
            Header<
                StaticArg<"User-Agent">,
                StaticArg<"hypershell">,
            >
        ],
    >
    | DecodeJson<Vec<Issue>>
};

#[cgp_context(MyAppComponents: HypershellPreset)]
#[derive(HasField)]
pub struct MyApp {
    pub http_client: Client,
    pub base_url: String,
    pub github_org: String,
    pub github_repo: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Issue {
    pub id: u64,
    pub state: String,
    pub title: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = MyApp {
        http_client: Client::new(),
        base_url: "https://api.github.com".to_owned(),
        github_org: "rust-lang".to_owned(),
        github_repo: "rust".to_owned(),
    };

    let response = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("List of GitHub issues: {response:#?}");

    Ok(())
}
