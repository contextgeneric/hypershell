use cgp::core::component::IsDelegateKeyIn;
use cgp::prelude::*;

use crate::components::{
    CommandArgExtractorComponent, StringArgExtractorComponent, UrlArgExtractorComponent,
};
use crate::dsl::{
    BytesToStream, BytesToString, ConvertTo, FieldArg, JoinArgs, Pipe, ReadFile, SimpleExec,
    SimpleHttpRequest, StaticArg, StreamToBytes, StreamToStdout, StreamToString, StreamingExec,
    StreamingHttpRequest, UrlEncodeArg, Use, WebSocket, WriteFile,
};

cgp_namespace! {
    new HypershellNamespace: DefaultNamespace {
        @cgp.extra.HandlerComponent.<T: IsDelegateKeyIn<HypershellHandlers>> T:
            UseDelegate<HypershellHandlers>,

        @cgp.extra.StringArgExtractorComponent.<T: IsDelegateKeyIn<HypershellStringArgExtractors>> T:
            UseDelegate<HypershellHandlers>,

        @cgp.extra.CommandArgExtractorComponent.<T: IsDelegateKeyIn<HypershellCommandArgExtractors>> T:
            UseDelegate<HypershellHandlers>,

        @cgp.extra.UrlArgExtractorComponent.<T: IsDelegateKeyIn<HypershellCommandArgExtractors>> T:
            UseDelegate<HypershellUrlArgExtractors>,
    }
}

delegate_components! {
    new HypershellHandlers {
        [
            <Handlers> Pipe<Handlers>,
            <Provider, Code> Use<Provider, Code>,
        ] =>
            @hypershell.dsl.handler.core,

        [
            BytesToString,
            <T> ConvertTo<T>,
        ] =>
            @hypershell.dsl.handler.convert,

        [
            StreamToBytes,
            StreamToString,
            BytesToStream,
            StreamToStdout,
        ] =>
            @hypershell.dsl.handler.stream,

        [
            <Path, Args> SimpleExec<Path, Args>,
            <Path, Args> StreamingExec<Path, Args>,
        ] =>
            @hypershell.dsl.handler.cli,

        [
            <Path> ReadFile<Path>,
            <Path> WriteFile<Path>,
        ] =>
            @hypershell.dsl.handler.file,

        [
            <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>,
            <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>,
        ] =>
            @hypershell.dsl.handler.http,

        [
            <Url, Params> WebSocket<Url, Params>,
        ] =>
            @hypershell.dsl.handler.websocket,

    }
}

delegate_components! {
    new HypershellStringArgExtractors {
        [
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
            <Args> JoinArgs<Args>,
        ] =>
            @hypershell.dsl.arg.core,
        [
            <Arg> UrlEncodeArg<Arg>,
        ] =>
            @hypershell.dsl.arg.url,
    }
}

delegate_components! {
    new HypershellCommandArgExtractors {
        [
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
        ] =>
            @hypershell.dsl.arg.core
    }
}

delegate_components! {
    new HypershellUrlArgExtractors {
        [
            <Arg> StaticArg<Arg>,
            <Tag> FieldArg<Tag>,
            <Args> JoinArgs<Args>,
        ] =>
            @hypershell.dsl.arg.core
    }
}
