use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::HttpApp;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    DecodeJson, EncodeJson, FieldArg, GetMethod, Header, JoinArgs, Pipe, PostMethod,
    SimpleHttpRequest, StaticArg, UrlEncodeArg, WithHeaders,
};
use hypershell_macro::hypershell;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn test_basic_http_request() -> Result<(), Error> {
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

    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
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

    let app = TestApp {
        http_client: Client::new(),
        base_url: "https://api.github.com".to_owned(),
        github_org: "rust-lang".to_owned(),
        github_repo: "rust".to_owned(),
    };

    let response = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("List of GitHub issues: {response:#?}");

    Ok(())
}

#[tokio::test]
async fn test_post_http_request() -> Result<(), Error> {
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
    pub struct Input {
        pub code: String,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    pub struct Response {
        pub id: String,
        pub url: String,
        pub code: String,
    }

    let app = HttpApp {
        http_client: Client::new(),
    };

    let input = Input {
        code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
    };

    let output = app.handle(PhantomData::<Program>, input).await?;

    println!("response: {output:#?}");

    Ok(())
}
