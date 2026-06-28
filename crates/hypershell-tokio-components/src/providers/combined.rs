use std::path::PathBuf;

use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_components::components::{
    CommandArgExtractorComponent, CommandArgTypeProviderComponent, StringArgExtractorComponent,
    UrlArgExtractorComponent,
};
use hypershell_components::dsl::{
    BytesToStream, FieldArgs, JoinArgs, ReadFile, SimpleExec, StreamToBytes, StreamToStdout,
    StreamToString, StreamingExec, WithArgs, WriteFile,
};

use crate::components::CommandUpdaterComponent;
use crate::dsl::{CoreExec, ToTokioAsyncRead};
use crate::providers::{
    ExtractArgs, ExtractFieldArgs, HandleBytesToTokioAsyncRead, HandleCoreExec, HandleReadFile,
    HandleSimpleExec, HandleStreamToStdout, HandleStreamingExec, HandleToTokioAsyncRead,
    HandleTokioAsyncReadToBytes, HandleTokioAsyncReadToString, HandleWriteFile, JoinExtractArgs,
    WrapTokioAsyncRead,
};

delegate_components! {
    new HypershellTokioProvider {
        open {
            HandlerComponent,
            StringArgExtractorComponent,
            CommandArgExtractorComponent,
            UrlArgExtractorComponent,
            CommandUpdaterComponent,
        };

        CommandArgTypeProviderComponent:
            UseType<PathBuf>,

        @HandlerComponent.<Path, Args> SimpleExec<Path, Args>:
            HandleSimpleExec,

        @HandlerComponent.<Path, Args> StreamingExec<Path, Args>:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                HandleStreamingExec,
                WrapTokioAsyncRead,
            ]>,

        @HandlerComponent.<Path, Args> CoreExec<Path, Args>:
            HandleCoreExec,

        @HandlerComponent.<Path> ReadFile<Path>:
            PipeHandlers<Product![
                HandleReadFile,
                WrapTokioAsyncRead,
            ]>,

        @HandlerComponent.<Path> WriteFile<Path>:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                HandleWriteFile,
            ]>,

        @HandlerComponent.StreamToBytes:
            HandleTokioAsyncReadToBytes,

        @HandlerComponent.StreamToString:
            HandleTokioAsyncReadToString,

        @HandlerComponent.BytesToStream:
            HandleBytesToTokioAsyncRead,

        @HandlerComponent.StreamToStdout:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                HandleStreamToStdout,
            ]>,

        @HandlerComponent.ToTokioAsyncRead:
            HandleToTokioAsyncRead,

        @CommandArgExtractorComponent.<Args> JoinArgs<Args>:
            JoinExtractArgs,

        @CommandUpdaterComponent.<Args> WithArgs<Args>:
            ExtractArgs,

        @CommandUpdaterComponent.<Tag> FieldArgs<Tag>:
            ExtractFieldArgs,
    }
}
