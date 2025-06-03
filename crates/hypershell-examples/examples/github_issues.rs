use hypershell::prelude::*;
use hypershell::presets::HypershellAppPreset;
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

#[tokio::main]
async fn main() -> Result<(), Error> {
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
