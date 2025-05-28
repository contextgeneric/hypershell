use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    BytesToJson, FieldArg, GetMethod, Header, JoinArgs, Pipe, SimpleHttpRequest, StaticArg,
    UrlEncodeArg, WithHeaders,
};
use reqwest::Client;
use serde::Deserialize;

#[tokio::test]
async fn test_basic_http_request() -> Result<(), Error> {
    pub type Program = Pipe<
        Product![
            SimpleHttpRequest<
                GetMethod,
                JoinArgs<Product![
                    FieldArg<symbol!("base_url")>,
                    StaticArg<symbol!("/repos/")>,
                    UrlEncodeArg<FieldArg<symbol!("github_org")>>,
                    StaticArg<symbol!("/")>,
                    UrlEncodeArg<FieldArg<symbol!("github_repo")>>,
                    StaticArg<symbol!("/issues")>,
                ]>,
                WithHeaders<Product![
                    Header<
                        StaticArg<symbol!("User-Agent")>,
                        StaticArg<symbol!("hypershell")>,
                    >
                ]>,
            >,
            BytesToJson<Vec<Issue>>,
        ],
    >;

    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub http_client: Client,
        pub base_url: String,
        pub github_org: String,
        pub github_repo: String,
    }

    let app = TestApp {
        http_client: Client::new(),
        base_url: "https://api.github.com".to_owned(),
        github_org: "rust-lang".to_owned(),
        github_repo: "rust".to_owned(),
    };

    let response = app.handle(PhantomData::<Program>, Vec::new()).await?;

    println!("HTTP response: {response:#?}");

    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub state: String,
    pub title: String,
}
