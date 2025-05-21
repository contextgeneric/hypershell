use cgp::core::component::UseDelegate;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::extra::handler::HandlerComponent;
use cgp::prelude::*;
use cgp_error_anyhow::{DebugAnyhowError, UseAnyhowError};
use hypershell_components::components::ArgExtractorComponent;
use hypershell_components::dsl::{Pipe, SimpleExec, StaticArg};
use hypershell_components::providers::{ExtractFieldArg, ExtractStaticArg, RunPipe};
use hypershell_tokio_components::components::CommandUpdaterComponent;
use hypershell_tokio_components::providers::{ExtractArgs, RunSimpleExec};

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
        CommandUpdaterComponent:
            ExtractArgs,
    }
}

delegate_components! {
    new HandlerComponents {
        <Handlers> Pipe<Handlers>:
            RunPipe,
        <Path, Args> SimpleExec<Path, Args>:
            RunSimpleExec,
    }
}

delegate_components! {
    new ArgExtractorComponents {
        <Arg> StaticArg<Arg>: ExtractStaticArg,
        <Tag> UseField<Tag>: ExtractFieldArg,
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
