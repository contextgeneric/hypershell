use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_components::components::{
    CommandArgExtractorComponent, HttpMethodTypeProviderComponent, MethodArgExtractorComponent,
    StringArgExtractorComponent, UrlArgExtractorComponent, UrlTypeProviderComponent,
};
use hypershell_components::dsl::{
    DeleteMethod, GetMethod, Header, PostMethod, PutMethod, SimpleHttpRequest,
    StreamingHttpRequest, UrlEncodeArg, WithHeaders,
};
use hypershell_tokio_components::providers::{HandleToTokioAsyncRead, WrapFuturesAsyncRead};
use hypershell_tokio_components::types::{FuturesAsyncReadStream, TokioAsyncReadStream};
use reqwest::Method;
use url::Url;

use crate::components::RequestBuilderUpdaterComponent;
use crate::dsl::CoreHttpRequest;
use crate::providers::{
    ExtractReqwestMethod, HandleCoreHttpRequest, HandleSimpleHttpRequest,
    HandleStreamingHttpRequest, StreamToBody, UpdateRequestHeader, UpdateRequestHeaders,
    UrlEncodeStringArg,
};

delegate_components! {
    new HypershellReqwestProvider {
        open {
            HandlerComponent,
            StringArgExtractorComponent,
            CommandArgExtractorComponent,
            UrlArgExtractorComponent,
            MethodArgExtractorComponent,
            RequestBuilderUpdaterComponent,
        };

        HttpMethodTypeProviderComponent:
            UseType<Method>,

        UrlTypeProviderComponent:
            UseType<Url>,

        @HandlerComponent.<Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>:
            HandleSimpleHttpRequest,

        // A byte buffer is sent as a buffered body, which `reqwest` can resend when it follows a
        // redirect. A streamed body cannot be resent, so a reader input does not follow one.
        @HandlerComponent
            .<Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>
            .[Vec<u8>, String]:
            PipeHandlers<Product![
                HandleStreamingHttpRequest,
                WrapFuturesAsyncRead,
            ]>,

        @HandlerComponent
            .<Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>
            .[<S> TokioAsyncReadStream<S>, <S> FuturesAsyncReadStream<S>]:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                StreamToBody,
                HandleStreamingHttpRequest,
                WrapFuturesAsyncRead,
            ]>,

        @HandlerComponent.<Method, Url, Headers> CoreHttpRequest<Method, Url, Headers>:
            HandleCoreHttpRequest,

        @MethodArgExtractorComponent.[
            GetMethod,
            PostMethod,
            PutMethod,
            DeleteMethod,
        ]:
            ExtractReqwestMethod,

        @StringArgExtractorComponent.<Arg> UrlEncodeArg<Arg>:
            UrlEncodeStringArg,

        @RequestBuilderUpdaterComponent.<Args> WithHeaders<Args>:
            UpdateRequestHeaders,

        @RequestBuilderUpdaterComponent.<Key, Value> Header<Key, Value>:
            UpdateRequestHeader,
    }
}
