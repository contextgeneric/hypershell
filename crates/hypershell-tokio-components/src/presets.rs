#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, PipeHandlers};
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        BytesToStream, FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg,
        StreamToBytes, StreamToStdout, StreamToString, StreamingExec, WithArgs, WriteFile,
    };
    use hypershell_components::providers::ExtractStringCommandArg;

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::{CoreExec, ToTokioAsyncRead};
    use crate::providers::{
        ExtractArgs, ExtractFieldArgs, HandleBytesToTokioAsyncRead, HandleCoreExec, HandleReadFile,
        HandleSimpleExec, HandleStreamToStdout, HandleStreamingExec, HandleToTokioAsyncRead,
        HandleTokioAsyncReadToBytes, HandleTokioAsyncReadToString, HandleWriteFile,
        JoinExtractArgs, WrapTokioAsyncRead,
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
                PipeHandlers<Product![
                    HandleToTokioAsyncRead,
                    HandleStreamingExec,
                    WrapTokioAsyncRead,
                ]>,
            <Path, Args> CoreExec<Path, Args>:
                HandleCoreExec,
            <Path> ReadFile<Path>:
                PipeHandlers<Product![
                    HandleReadFile,
                    WrapTokioAsyncRead,
                ]>,
            <Path> WriteFile<Path>:
                PipeHandlers<Product![
                    HandleToTokioAsyncRead,
                    HandleWriteFile,
                ]>,
            StreamToBytes:
                HandleTokioAsyncReadToBytes,
            StreamToString:
                HandleTokioAsyncReadToString,
            BytesToStream:
                HandleBytesToTokioAsyncRead,
            StreamToStdout:
                PipeHandlers<Product![
                    HandleToTokioAsyncRead,
                    HandleStreamToStdout,
                ]>,
            ToTokioAsyncRead:
                HandleToTokioAsyncRead,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        CommandArgExtractorPreset {
            [
                <Arg> StaticArg<Arg>,
                <Tag> FieldArg<Tag>,
            ]: ExtractStringCommandArg,
            <Args> JoinArgs<Args>:
                JoinExtractArgs,
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
