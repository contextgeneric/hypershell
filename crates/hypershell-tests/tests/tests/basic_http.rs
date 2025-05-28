use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::presets::HypershellAppPreset;
use hypershell_components::dsl::{
    BytesToString, FieldArg, GetMethod, Header, JoinArgs, Pipe, SimpleHttpRequest, StaticArg,
    UrlEncodeArg, WithHeaders,
};
use reqwest::Client;

#[tokio::test]
async fn test_basic_http_request() -> Result<(), Error> {
    #[cgp_context(TestAppComponents: HypershellAppPreset)]
    #[derive(HasField)]
    pub struct TestApp {
        pub http_client: Client,
        pub github_org: String,
        pub github_repo: String,
    }

    let app = TestApp {
        http_client: Client::new(),
        github_org: "contextgeneric".to_owned(),
        github_repo: "cgp".to_owned(),
    };

    let response = app
        .handle(
            PhantomData::<
                Pipe<
                    Product![
                        SimpleHttpRequest<
                            GetMethod,
                            JoinArgs<Product![
                                StaticArg<symbol!("https://api.github.com/repos/")>,
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
                        BytesToString,
                    ],
                >,
            >,
            Vec::new(),
        )
        .await?;

    println!("HTTP response: {response}");

    Ok(())
}
