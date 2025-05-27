#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg, WithArgs,
    };
    use hypershell_components::providers::{ExtractStringCommandArg, Run};

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
                TokioHandlerPreset::Provider,
            CommandArgExtractorComponent:
                UseDelegate<CommandArgExtractorPreset::Provider>,
            CommandUpdaterComponent:
                UseDelegate<CommandUpdaterPreset::Provider>,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        TokioHandlerPreset {
            <Path, Args> SimpleExec<Path, Args>:
                RunSimpleExec,
            <Path, Args> CoreExec<Path, Args>:
                RunCoreExec,
            <Path> ReadFile<Path>:
                Run<SimpleExec<StaticArg<symbol!("cat")>, WithArgs<Product![Path]>>>,
        }
    }

    cgp_preset! {
        CommandArgExtractorPreset {
            [
                <Arg> StaticArg<Arg>,
                <Tag> FieldArg<Tag>,
            ]: ExtractStringCommandArg,
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
