use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use cgp_error_anyhow::{DebugAnyhowError, UseAnyhowError};
use hypershell_components::dsl::{SimpleExec, StaticArg};
use hypershell_tokio_components::presets::HypershellTokioPreset;

#[cgp_context(HypershellAppComponents: HypershellTokioPreset)]
pub struct HypershellApp;

delegate_components! {
    HypershellAppComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            DebugAnyhowError,
    }
}

check_components! {
    CanUseHypershellApp for HypershellApp {
        HandlerComponent: [
            (
                SimpleExec<
                    StaticArg<symbol!("echo")>,
                    Product! [
                        StaticArg<symbol!("hello")>
                    ],
                >,
                Vec<u8>,
            )
        ]
    }
}
