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
        BytesToStream, FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg,
        StreamToBytes, StreamToString, StreamingExec, WithArgs,
    };
    use hypershell_components::providers::ExtractStringCommandArg;

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::CoreExec;
    use crate::providers::{
        ConvertStream, ExtractArgs, ExtractFieldArgs, HandleCoreExec, HandleReadFile,
        HandleSimpleExec, HandleStreamingExec, JoinExtractArgs,
    };

    cgp_preset! {
        HypershellTokioPreset {
            CommandArgTypeProviderComponent:
                UseType<PathBuf>,
            HandlerComponent:
                TokioHandlerPreset::Provider,
            CommandArgExtractorComponent:
                CommandArgExtractorPreset::Provider,
            CommandUpdaterComponent:
                CommandUpdaterPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        TokioHandlerPreset {
            <Path, Args> SimpleExec<Path, Args>:
                HandleSimpleExec,
            <Path, Args> StreamingExec<Path, Args>:
                HandleStreamingExec,
            <Path, Args> CoreExec<Path, Args>:
                HandleCoreExec,
            <Path> ReadFile<Path>:
                HandleReadFile,
            [
                StreamToBytes,
                StreamToString,
                BytesToStream,
            ]:
                ConvertStream,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        CommandArgExtractorPreset {
            [
                <Arg> StaticArg<Arg>,
                <Tag> FieldArg<Tag>,
            ]: ExtractStringCommandArg,
            <Args> JoinArgs<Args>: JoinExtractArgs,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        CommandUpdaterPreset {
            <Args> WithArgs<Args>: ExtractArgs,
            <Tag> FieldArgs<Tag>: ExtractFieldArgs,
        }
    }
}
