#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::components::ArgExtractorComponent;
    use hypershell_components::dsl::{Pipe, SimpleExec, StaticArg};
    use hypershell_components::providers::{ExtractFieldArg, ExtractStaticArg, RunPipe};

    use crate::components::CommandUpdaterComponent;
    use crate::providers::{ExtractArgs, RunSimpleExec};

    cgp_preset! {
        HypershellTokioPreset {
            HandlerComponent:
                UseDelegate<HandlerComponents::Provider>,
            ArgExtractorComponent:
                UseDelegate<ArgExtractorComponents::Provider>,
            CommandUpdaterComponent:
                ExtractArgs,
        }
    }

    cgp_preset! {
        HandlerComponents {
            <Handlers> Pipe<Handlers>:
                RunPipe,
            <Path, Args> SimpleExec<Path, Args>:
                RunSimpleExec,
        }
    }

    cgp_preset! {
        ArgExtractorComponents {
            <Arg> StaticArg<Arg>: ExtractStaticArg,
            <Tag> UseField<Tag>: ExtractFieldArg,
        }
    }
}
