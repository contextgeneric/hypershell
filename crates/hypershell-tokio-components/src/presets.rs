#[cgp::re_export_imports]
mod preset {
    use core::pin::Pin;
    use std::path::PathBuf;
    use std::vec::Vec;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, PipeHandlers, UseInputDelegate};
    use cgp::prelude::{cgp_preset, *};
    use futures::AsyncRead;
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        BytesToStream, FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg,
        StreamToBytes, StreamToStdout, StreamToString, StreamingExec, WithArgs,
    };
    use hypershell_components::providers::{Call, ExtractStringCommandArg};
    use tokio::io::AsyncRead as TokioAsyncRead;

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::CoreExec;
    use crate::providers::{
        ConvertBytesToStream, ConvertStreamToBytes, ConvertStreamToString, ExtractArgs,
        ExtractFieldArgs, FuturesToTokioStream, HandleCoreExec, HandleReadFile, HandleSimpleExec,
        HandleStreamToStdout, HandleStreamingExec, JoinExtractArgs,
    };
    use crate::types::tokio_async_read::TokioAsyncReadStream;

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
                StreamingExecHandlers::Provider,
            <Path, Args> CoreExec<Path, Args>:
                HandleCoreExec,
            <Path> ReadFile<Path>:
                HandleReadFile,
            StreamToBytes:
                ConvertStreamToBytes,
            StreamToString:
                ConvertStreamToString,
            BytesToStream:
                ConvertBytesToStream,
            StreamToStdout:
                HandleStreamToStdout,
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

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        StreamingExecHandlers {
            Pin<Box<dyn AsyncRead + Send>>:
                PipeHandlers<Product![
                    FuturesToTokioStream,
                    HandleStreamingExec,
                ]>,
            <S> TokioAsyncReadStream<S>:
                HandleStreamingExec,
            Pin<Box<dyn TokioAsyncRead + Send>>:
                HandleStreamingExec,
            Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleStreamingExec,
                ]>,
        }
    }
}
