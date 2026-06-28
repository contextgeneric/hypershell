use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_components::components::{
    CommandArgExtractorComponent, MethodArgExtractorComponent, StringArgExtractorComponent,
    UrlArgExtractorComponent,
};
use hypershell_components::dsl::{
    GetMethod, Header, PostMethod, SimpleHttpRequest, StreamingHttpRequest, UrlEncodeArg,
    WithHeaders,
};
use hypershell_tokio_components::providers::{HandleToTokioAsyncRead, WrapFuturesAsyncRead};

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

        @HandlerComponent.<Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>:
            HandleSimpleHttpRequest,

        @HandlerComponent.<Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                StreamToBody,
                HandleStreamingHttpRequest,
                WrapFuturesAsyncRead,
            ]>,

        @HandlerComponent.<Method, Url, Headers> CoreHttpRequest<Method, Url, Headers>:
            HandleCoreHttpRequest,

        @UrlArgExtractorComponent.[
            GetMethod,
            PostMethod,
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
