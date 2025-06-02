#[cgp::re_export_imports]
mod preset {
    use core::pin::Pin;
    use std::vec::Vec;

    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::{HandlerComponent, PipeHandlers, UseInputDelegate};
    use cgp::prelude::{cgp_preset, *};
    use futures::AsyncRead;
    use hypershell_components::components::{
        HttpMethodTypeProviderComponent, MethodArgExtractorComponent, StringArgExtractorComponent,
        UrlTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        GetMethod, Header, PostMethod, SimpleHttpRequest, StreamingHttpRequest, UrlEncodeArg,
        WithHeaders,
    };
    use hypershell_tokio_components::providers::FuturesToTokioStream;
    use reqwest::{Method, Url};
    use tokio::io::AsyncRead as TokioAsyncRead;

    use crate::components::RequestBuilderUpdaterComponent;
    use crate::dsl::CoreHttpRequest;
    use crate::providers::{
        ExtractReqwestMethod, HandleCoreHttpRequest, HandleSimpleHttpRequest,
        HandleStreamingHttpRequest, StreamToBody, UpdateRequestHeader, UpdateRequestHeaders,
        UrlEncodeStringArg,
    };

    cgp_preset! {
        HypershellReqwestPreset {
            HandlerComponent:
                ReqwestHandlerPreset::Provider,
            HttpMethodTypeProviderComponent:
                UseType<Method>,
            UrlTypeProviderComponent:
                UseType<Url>,
            StringArgExtractorComponent:
                ReqwestStringArgExtractorPreset::Provider,
            MethodArgExtractorComponent:
                MethodArgExtractorPreset::Provider,
            RequestBuilderUpdaterComponent:
                RequestBuilderUpdaterPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        ReqwestHandlerPreset {
            <Method, Url, Headers> SimpleHttpRequest<Method, Url, Headers>:
                HandleSimpleHttpRequest,
            <Method, Url, Headers> StreamingHttpRequest<Method, Url, Headers>:
                StreamingHttpHandlers::Provider,
            <Method, Url, Headers> CoreHttpRequest<Method, Url, Headers>:
                HandleCoreHttpRequest,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        MethodArgExtractorPreset {
            [
                GetMethod,
                PostMethod,
            ]:
                ExtractReqwestMethod,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        ReqwestStringArgExtractorPreset {
            <Arg> UrlEncodeArg<Arg>:
                UrlEncodeStringArg,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        RequestBuilderUpdaterPreset {
            <Args> WithHeaders<Args>:
                UpdateRequestHeaders,
            <Key, Value> Header<Key, Value>:
                UpdateRequestHeader,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        StreamingHttpHandlers {
            Pin<Box<dyn AsyncRead + Send>>:
                PipeHandlers<Product![
                    FuturesToTokioStream,
                    StreamToBody,
                    HandleStreamingHttpRequest,
                ]>,
            Pin<Box<dyn TokioAsyncRead + Send>>:
                PipeHandlers<Product![
                    StreamToBody,
                    HandleStreamingHttpRequest,
                ]>,
            Vec<u8>:
                HandleStreamingHttpRequest,
        }
    }
}
