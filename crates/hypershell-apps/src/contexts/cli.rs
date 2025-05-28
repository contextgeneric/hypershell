use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use hypershell_components::dsl::{SimpleExec, StaticArg, WithArgs};

use crate::presets::HypershellAppPreset;

#[cgp_context(CliAppComponents: HypershellAppPreset)]
pub struct CliApp;

check_components! {
    CanUseHypershellApp for CliApp {
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
        ],
    }
}
