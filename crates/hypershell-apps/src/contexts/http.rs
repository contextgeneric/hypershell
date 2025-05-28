use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use hypershell_components::components::{StringArgExtractorComponent, UrlArgExtractorComponent};
use hypershell_components::dsl::{
    GetMethod, Header, JoinArgs, SimpleExec, SimpleHttpRequest, StaticArg, WithArgs, WithHeaders,
};
use hypershell_reqwest_components::components::RequestBuilderUpdaterComponent;
use reqwest::Client;

use crate::presets::HypershellAppPreset;

#[cgp_context(HttpAppComponents: HypershellAppPreset)]
#[derive(HasField)]
pub struct HttpApp {
    pub http_client: Client,
}

check_components! {
    CanUseHypershellApp for HttpApp {
        HandlerComponent: [
            (
                SimpleExec<
                    StaticArg<symbol!("echo")>,
                    WithArgs<Product! [
                        StaticArg<symbol!("hello")>
                    ]>,
                >,
                Vec<u8>,
            ),
            (
                SimpleHttpRequest<
                    GetMethod,
                    StaticArg<symbol!("http://example.org")>,
                    WithHeaders<Nil>,
                >,
                Vec<u8>,
            ),
        ],
        [
            UrlArgExtractorComponent,
            StringArgExtractorComponent,
        ]:
            [
                StaticArg<symbol!("http://example.org")>,
                JoinArgs<Product![
                    StaticArg<symbol!("http://example.org")>
                ]>,
            ],
        RequestBuilderUpdaterComponent:
            Header<
                StaticArg<symbol!("User-Agent")>,
                StaticArg<symbol!("curl/8.5.0")>,
            >,
    }
}
