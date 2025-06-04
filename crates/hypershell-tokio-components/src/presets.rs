#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;
    use std::vec::Vec;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, PipeHandlers, UseInputDelegate};
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        BytesToStream, FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg,
        StreamToBytes, StreamToStdout, StreamToString, StreamingExec, WithArgs,
    };
    use hypershell_components::providers::{Call, ExtractStringCommandArg};

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::CoreExec;
    use crate::providers::{
        ConvertBytesToStream, ConvertStreamToBytes, ConvertStreamToString, ExtractArgs,
        ExtractFieldArgs, FuturesToTokioAsyncRead, HandleCoreExec, HandleReadFile,
        HandleSimpleExec, HandleStreamToStdout, HandleStreamingExec, JoinExtractArgs,
        WrapTokioAsyncRead,
    };
    use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

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
                PipeHandlers<Product![
                    HandleReadFile,
                    WrapTokioAsyncRead,
                ]>,
            StreamToBytes:
                ConvertStreamToBytes,
            StreamToString:
                ConvertStreamToString,
            BytesToStream:
                ConvertBytesToStream,
            StreamToStdout:
                StreamingToStdoutHandlers::Provider,
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
        StreamingToStdoutHandlers {
            <S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    HandleStreamToStdout,
                ]>,
            <S> TokioAsyncReadStream<S>:
                HandleStreamToStdout,
            Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleStreamToStdout,
                ]>,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        StreamingExecHandlers {
            <S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    HandleStreamingExec,
                    WrapTokioAsyncRead,
                ]>,
            <S> TokioAsyncReadStream<S>:
                PipeHandlers<Product![
                    HandleStreamingExec,
                    WrapTokioAsyncRead,
                ]>,
            Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleStreamingExec,
                    WrapTokioAsyncRead,
                ]>,
        }
    }
}
