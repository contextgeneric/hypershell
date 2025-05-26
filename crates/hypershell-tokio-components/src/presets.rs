#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent, StringArgExtractorComponent,
    };
    use hypershell_components::dsl::{
        BytesToString, FieldArg, FieldArgs, JoinArgs, Pipe, ReadFile, SimpleExec, StaticArg,
        WithArgs,
    };
    use hypershell_components::providers::{
        DecodeUtf8Bytes, ExtractFieldArg, ExtractStaticArg, ExtractStringCommandArg, Run, RunPipe,
    };

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
            StringArgExtractorComponent:
                UseDelegate<StringArgExtractorPreset::Provider>,
            CommandArgExtractorComponent:
                UseDelegate<CommandArgExtractorPreset::Provider>,
            CommandUpdaterComponent:
                UseDelegate<CommandUpdaterPreset::Provider>,
        }
    }

    cgp_preset! {
        HandlerPreset {
            BytesToString:
                DecodeUtf8Bytes,
            <Handlers> Pipe<Handlers>:
                RunPipe,
            <Path, Args> SimpleExec<Path, Args>:
                RunSimpleExec,
            <Path, Args> CoreExec<Path, Args>:
                RunCoreExec,
            <Path> ReadFile<Path>:
                Run<SimpleExec<StaticArg<symbol!("cat")>, WithArgs<Product![Path]>>>,
        }
    }

    cgp_preset! {
        StringArgExtractorPreset {
            <Arg> StaticArg<Arg>: ExtractStaticArg,
            <Tag> FieldArg<Tag>: ExtractFieldArg,
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
