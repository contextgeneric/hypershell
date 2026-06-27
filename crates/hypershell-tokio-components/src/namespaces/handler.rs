use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
use cgp::prelude::*;
use hypershell_components::dsl::{
    BytesToStream, ReadFile, SimpleExec, StreamToBytes, StreamToStdout, StreamToString,
    StreamingExec, WriteFile,
};

use crate::dsl::{CoreExec, ToTokioAsyncRead};
use crate::providers::{
    HandleBytesToTokioAsyncRead, HandleCoreExec, HandleReadFile, HandleSimpleExec,
    HandleStreamToStdout, HandleStreamingExec, HandleToTokioAsyncRead, HandleTokioAsyncReadToBytes,
    HandleTokioAsyncReadToString, HandleWriteFile, WrapTokioAsyncRead,
};

cgp_namespace! {
    new TokioHandlerImpls {
        <Path, Args> SimpleExec<Path, Args>:
            HandleSimpleExec,
        <Path, Args> StreamingExec<Path, Args>:
            PipeHandlers<Product![
                UseInputDelegate<HandleToTokioAsyncRead>,
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
                UseInputDelegate<HandleToTokioAsyncRead>,
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
                UseInputDelegate<HandleToTokioAsyncRead>,
                HandleStreamToStdout,
            ]>,
        ToTokioAsyncRead:
            UseInputDelegate<HandleToTokioAsyncRead>,
        <Path, Args> CoreExec<Path, Args>:
            HandleCoreExec,
    }
}
