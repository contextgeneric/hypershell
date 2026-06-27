use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
use cgp::prelude::*;
use hypershell_components::dsl::{
    BytesToStream, ReadFile, SimpleExec, StreamToBytes, StreamToStdout, StreamToString,
    StreamingExec, WriteFile,
};
use hypershell_components::providers::ReturnInput;

use crate::dsl::{CoreExec, ToTokioAsyncRead};
use crate::providers::{
    FuturesToTokioAsyncRead, HandleBytesToTokioAsyncRead, HandleCoreExec, HandleReadFile,
    HandleSimpleExec, HandleStreamToStdout, HandleStreamingExec, HandleTokioAsyncReadToBytes,
    HandleTokioAsyncReadToString, HandleWriteFile, WrapTokioAsyncRead,
};
use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

cgp_namespace! {
    new TokioHandlerImpls {
        <Path, Args> SimpleExec<Path, Args>:
            HandleSimpleExec,
        <Path, Args> StreamingExec<Path, Args>:
            PipeHandlers<Product![
                UseInputDelegate<ToTokioAsyncReadHandlers>,
                HandleStreamingExec,
                WrapTokioAsyncRead,
            ]>,
        <Path> ReadFile<Path>:
            PipeHandlers<Product![
                HandleReadFile,
                WrapTokioAsyncRead,
            ]>,
        <Path> WriteFile<Path>:
            PipeHandlers<Product![
                UseInputDelegate<ToTokioAsyncReadHandlers>,
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
                UseInputDelegate<ToTokioAsyncReadHandlers>,
                HandleStreamToStdout,
            ]>,
        ToTokioAsyncRead:
            UseInputDelegate<ToTokioAsyncReadHandlers>,
        <Path, Args> CoreExec<Path, Args>:
            HandleCoreExec,
    }
}

delegate_components! {
    new ToTokioAsyncReadHandlers {
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
