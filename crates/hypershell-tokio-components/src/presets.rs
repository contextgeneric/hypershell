#[cgp::re_export_imports]
mod preset {
    use std::path::PathBuf;
    use std::string::String;
    use std::vec::Vec;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, PipeHandlers, UseInputDelegate};
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::{
        CommandArgExtractorComponent, CommandArgTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        BytesToStream, FieldArg, FieldArgs, JoinArgs, ReadFile, SimpleExec, StaticArg,
        StreamToBytes, StreamToStdout, StreamToString, StreamingExec, WithArgs, WriteFile,
    };
    use hypershell_components::providers::{ExtractStringCommandArg, ReturnInput};

    use crate::components::CommandUpdaterComponent;
    use crate::dsl::{CoreExec, ToTokioAsyncRead};
    use crate::providers::{
        AsyncReadToStream, ExtractArgs, ExtractFieldArgs, FuturesToTokioAsyncRead,
        HandleBytesToStream, HandleBytesToTokioAsyncRead, HandleCoreExec, HandleReadFile,
        HandleSimpleExec, HandleStreamToStdout, HandleStreamingExec, HandleTokioAsyncReadToBytes,
        HandleTokioAsyncReadToString, HandleWriteFile, JoinExtractArgs, WrapTokioAsyncRead,
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
                PipeHandlers<Product![
                    ToTokioAsyncReadHandlers::Provider,
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
                    ToTokioAsyncReadHandlers::Provider,
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
                    ToTokioAsyncReadHandlers::Provider,
                    HandleStreamToStdout,
                ]>,
            ToTokioAsyncRead:
                ToTokioAsyncReadHandlers::Provider,
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
        ToTokioAsyncReadHandlers {
            <S> FuturesAsyncReadStream<S>:
                FuturesToTokioAsyncRead,
            <S> TokioAsyncReadStream<S>:
                ReturnInput,
            [
                Vec<u8>,
                String,
            ]:
                HandleBytesToTokioAsyncRead,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        ToFuturesStreamHandlers {
            <S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    AsyncReadToStream,
                ]>,
            <S> TokioAsyncReadStream<S>:
                AsyncReadToStream,
            [
                Vec<u8>,
                String,
            ]:
                HandleBytesToStream,
        }
    }
}
