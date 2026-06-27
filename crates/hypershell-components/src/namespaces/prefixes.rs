use alloc::boxed::Box;

use cgp::prelude::*;

use crate::components::{StringArgExtractorComponent, UrlArgExtractorComponent};
use crate::dsl::{
    BytesToStream, BytesToString, ConvertTo, FieldArg, JoinArgs, Pipe, ReadFile, SimpleExec, SimpleHttpRequest, StaticArg, StreamToBytes, StreamToStdout, StreamToString, StreamingExec, StreamingHttpRequest, Use, WebSocket, WriteFile,
};

cgp_namespace! {
    new HypershellNamespace: DefaultNamespace {
        @cgp.extra.HandlerComponent.{
            <Handlers> Pipe<Handlers>,
            <Provider, Code> Use<Provider, Code>,
            <Code> Box<Code>,
        } =>
            @hypershell.dsl.handler.core,

        @cgp.extra.HandlerComponent.{
            BytesToString,
            <T> ConvertTo<T>,
        } =>
            @hypershell.dsl.handler.convert,

        @cgp.extra.HandlerComponent.{
            StreamToBytes,
            StreamToString,
            BytesToStream,
            StreamToStdout,
        } =>
            @hypershell.dsl.handler.stream,

        @cgp.extra.HandlerComponent.{
            <Path, Args> SimpleExec<Path, Args>,
            <Path, Args> StreamingExec<Path, Args>,
        } =>
            @hypershell.dsl.handler.cli,

        @cgp.extra.HandlerComponent.{
            <Path> ReadFile<Path>,
            <Path> WriteFile<Path>,
        } =>
            @hypershell.dsl.handler.file,


        @cgp.extra.HandlerComponent.{
            <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>,
            <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>,
        } =>
            @hypershell.dsl.handler.http,

        @cgp.extra.HandlerComponent.{
            <Url, Params> WebSocket<Url, Params>,
        } =>
            @hypershell.dsl.handler.websocket,

        @hypershell.core.[StringArgExtractorComponent, UrlArgExtractorComponent].{
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
            <Args> JoinArgs<Args>,
        } =>
            @hypershell.dsl.arg.core,
    }
}
