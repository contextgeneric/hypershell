use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use hypershell_components::components::{StringArgExtractorComponent, UrlArgExtractorComponent};
use hypershell_components::dsl::{
    GetMethod, SimpleExec, SimpleHttpRequest, StaticArg, WithArgs, WithHeaders,
};
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
            StaticArg<symbol!("http://example.org")>,
    }
}
