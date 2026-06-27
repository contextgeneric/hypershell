use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_components::dsl::{SimpleHttpRequest, StreamingHttpRequest};
use hypershell_components::namespaces::BaseHandlerImpls;
use hypershell_tokio_components::namespaces::TokioHandlerImpls;
use hypershell_tokio_components::providers::{HandleToTokioAsyncRead, WrapFuturesAsyncRead};

use crate::dsl::CoreHttpRequest;
use crate::providers::{
    HandleCoreHttpRequest, HandleSimpleHttpRequest, HandleStreamingHttpRequest, StreamToBody,
};

cgp_namespace! {
    BaseHandlerImpls {
        <Method, Url, Headers> CoreHttpRequest<Method, Url, Headers>:
            HandleCoreHttpRequest,
    }
}

cgp_namespace! {
    new ReqwestHandlerImpls: TokioHandlerImpls {
        <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>:
            HandleSimpleHttpRequest,
        <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>:
            PipeHandlers<Product![
                HandleToTokioAsyncRead,
                StreamToBody,
                HandleStreamingHttpRequest,
                WrapFuturesAsyncRead,
            ]>,
    }
}
