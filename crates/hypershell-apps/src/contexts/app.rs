use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use cgp_error_anyhow::{DebugAnyhowError, UseAnyhowError};
use hypershell_components::components::ArgExtractorComponent;
use hypershell_components::dsl::StaticArg;
use hypershell_components::providers::ExtractStaticArg;

#[cgp_context]
pub struct HypershellApp {}

delegate_components! {
    HypershellAppComponents {
        ErrorTypeProviderComponent:
            UseAnyhowError,
        ErrorRaiserComponent:
            DebugAnyhowError,
        HandlerComponent:
            UseDelegate<HandlerComponents>,
        ArgExtractorComponent:
            UseDelegate<ArgExtractorComponents>,
    }
}

delegate_components! {
    new HandlerComponents {

    }
}

delegate_components! {
    new ArgExtractorComponents {
        <Arg> StaticArg<Arg>: ExtractStaticArg,
    }
}
