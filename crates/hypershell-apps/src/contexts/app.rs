use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use hypershell_components::dsl::{
    GetMethod, SimpleExec, SimpleHttpRequest, StaticArg, WithArgs, WithHeaders,
};

use crate::presets::HypershellAppPreset;

#[cgp_context(HypershellAppComponents: HypershellAppPreset)]
pub struct HypershellApp;

check_components! {
    CanUseHypershellApp for HypershellApp {
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
            // (
            //     SimpleHttpRequest<
            //         GetMethod,
            //         StaticArg<symbol!("http://example.org")>,
            //         WithHeaders<Nil>,
            //     >,
            //     Vec<u8>,
            // ),
        ]
    }
}
