use core::marker::PhantomData;

use cgp::extra::handler::CanHandle;
use cgp::prelude::*;
use cgp_error_anyhow::Error;
use hypershell_apps::contexts::HttpApp;
use hypershell_components::dsl::{
    BytesToString, GetMethod, Header, Pipe, SimpleHttpRequest, StaticArg, WithHeaders,
};
use reqwest::Client;

#[tokio::test]
async fn test_basic_http_request() -> Result<(), Error> {
    let app = HttpApp {
        http_client: Client::new(),
    };

    let response = app
        .handle(
            PhantomData::<
                Pipe<
                    Product![
                        SimpleHttpRequest<
                            GetMethod,
                            StaticArg<symbol!("https://api.github.com/repos/contextgeneric/cgp/issues")>,
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
