#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::components::{
        ArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        FieldArg, FieldArgs, JoinArgs, Pipe, SimpleExec, StaticArg, WithArgs,
    };
    use hypershell_components::providers::{ExtractFieldArg, ExtractStaticArg, RunPipe};

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::CoreExec;
    use crate::providers::{
        ExtractArgs, ExtractFieldArgs, JoinExtractArgs, RunCoreExec, RunSimpleExec,
    };

    cgp_preset! {
        HypershellTokioPreset {
            CommandArgTypeProviderComponent:
                UseType<PathBuf>,
            HandlerComponent:
                UseDelegate<HandlerPreset::Provider>,
            ArgExtractorComponent:
                UseDelegate<ArgExtractorPreset::Provider>,
            CommandUpdaterComponent:
                UseDelegate<CommandUpdaterPreset::Provider>,
        }
    }

    cgp_preset! {
        HandlerPreset {
            <Handlers> Pipe<Handlers>:
                RunPipe,
            <Path, Args> SimpleExec<Path, Args>:
                RunSimpleExec,
            <Path, Args> CoreExec<Path, Args>:
                RunCoreExec,
        }
    }

    cgp_preset! {
        ArgExtractorPreset {
            <Arg> StaticArg<Arg>: ExtractStaticArg,
            <Tag> FieldArg<Tag>: ExtractFieldArg,
            <Args> JoinArgs<Args>: JoinExtractArgs,
        }
    }

    cgp_preset! {
        CommandUpdaterPreset {
            <Args> WithArgs<Args>: ExtractArgs,
            <Tag> FieldArgs<Tag>: ExtractFieldArgs,
        }
    }
}
