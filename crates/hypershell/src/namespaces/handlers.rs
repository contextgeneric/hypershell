use core::convert::Infallible;
use core::str::Utf8Error;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use cgp_error_anyhow::{DebugAnyhowError, Error, UseAnyhowError};
use hypershell_components::components::{
    CommandArgExtractorComponent, StringArgExtractorComponent, UrlArgExtractorComponent,
};
use hypershell_components::dsl::{
    BytesToStream, BytesToString, ConvertTo, DecodeJson, EncodeJson, FieldArg, FieldArgs,
    GetMethod, Header, JoinArgs, Pipe, PostMethod, ReadFile, SimpleExec, SimpleHttpRequest,
    StaticArg, StreamToBytes, StreamToStdout, StreamToString, StreamingExec, StreamingHttpRequest,
    UrlEncodeArg, Use, WithArgs, WithHeaders, WriteFile,
};
use hypershell_components::providers::HypershellBaseProvider;
use hypershell_json_components::providers::HypershellJsonProvider;
use hypershell_reqwest_components::components::{
    RequestBuilderUpdaterComponent, ReqwestClientGetterComponent,
};
use hypershell_reqwest_components::dsl::CoreHttpRequest;
use hypershell_reqwest_components::providers::{ErrorResponse, HypershellReqwestProvider};
use hypershell_tokio_components::components::CommandUpdaterComponent;
use hypershell_tokio_components::dsl::{CoreExec, ToTokioAsyncRead};
use hypershell_tokio_components::providers::{ExecOutputError, HypershellTokioProvider};
use reqwest::header::{InvalidHeaderName, InvalidHeaderValue};
use url::ParseError;

use crate::providers::HypershellErrorHandler;

cgp_namespace! {
    new HypershellNamespace: DefaultNamespace {
        @cgp.core.error.ErrorTypeProviderComponent:
            UseAnyhowError,

        @cgp.core.error.ErrorRaiserComponent.[
            Error,
            Infallible,
            std::io::Error,
            Utf8Error,
            reqwest::Error,
            ParseError,
            InvalidHeaderName,
            InvalidHeaderValue,
            serde_json::Error,
            ExecOutputError,
            ErrorResponse,
        ]:
            HypershellErrorHandler,

        @cgp.core.error.ErrorWrapperComponent:
            DebugAnyhowError,

        @hypershell.reqwest.ReqwestClientGetterComponent:
            UseField<Symbol!("http_client")>,

        @cgp.extra.handler.HandlerComponent.[
            BytesToString,
            <T> ConvertTo<T>,
            <Handlers> Pipe<Handlers>,
            <Provider, Code> Use<Provider, Code>,
            <Code> Box<Code>,
        ]:
            HypershellBaseProvider,

        @cgp.extra.handler.HandlerComponent.[
            <Path, Args> SimpleExec<Path, Args>,
            <Path, Args> StreamingExec<Path, Args>,
            <Path, Args> CoreExec<Path, Args>,
            <Path> ReadFile<Path>,
            <Path> WriteFile<Path>,
            StreamToBytes,
            StreamToString,
            BytesToStream,
            StreamToStdout,
            ToTokioAsyncRead,
        ]:
            HypershellTokioProvider,

        @cgp.extra.handler.HandlerComponent.[
            <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>,
            <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>,
            <Method, Url, Headers> CoreHttpRequest<Method, Url, Headers>,
        ]:
            HypershellReqwestProvider,

        @cgp.extra.handler.HandlerComponent.[
            EncodeJson,
            <Value> DecodeJson<Value>,
        ]:
            HypershellJsonProvider,

        @hypershell.core.{
            StringArgExtractorComponent.[
                <Arg> StaticArg<Arg>,
                <Tag> FieldArg<Tag>,
                <Args> JoinArgs<Args>,
            ],
            CommandArgExtractorComponent.[
                <Arg> StaticArg<Arg>,
                <Tag> FieldArg<Tag>,
            ],
            UrlArgExtractorComponent.[
                <Arg> StaticArg<Arg>,
                <Args> JoinArgs<Args>,
                <Tag> FieldArg<Tag>,
            ],
        }:
            HypershellBaseProvider,

        @hypershell.{
            core.CommandArgExtractorComponent.[
                <Args> JoinArgs<Args>,
            ],
            tokio.CommandUpdaterComponent.[
                <Args> WithArgs<Args>,
                <Tag> FieldArgs<Tag>,
            ],
        }:
            HypershellTokioProvider,

        @hypershell.{
            core.{
                StringArgExtractorComponent.[
                    <Arg> UrlEncodeArg<Arg>,
                ],
                UrlArgExtractorComponent.[
                    GetMethod,
                    PostMethod,
                ],
            },
            reqwest.RequestBuilderUpdaterComponent.[
                <Args> WithHeaders<Args>,
                <Key, Value> Header<Key, Value>,
            ],
        }:
            HypershellReqwestProvider,
    }
}
