#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::{
        HttpMethodTypeProviderComponent, MethodArgExtractorComponent, StringArgExtractorComponent,
        UrlTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        GetMethod, Header, PostMethod, SimpleHttpRequest, StreamingHttpRequest, UrlEncodeArg,
        WithHeaders,
    };
    use reqwest::{Method, Url};

    use crate::components::RequestBuilderUpdaterComponent;
    use crate::dsl::CoreHttpRequest;
    use crate::providers::{
        ExtractReqwestMethod, HandleCoreHttpRequest, HandleSimpleHttpRequest,
        HandleStreamingHttpRequest, UpdateRequestHeader, UpdateRequestHeaders, UrlEncodeStringArg,
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
                HandleStreamingHttpRequest,
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
}
