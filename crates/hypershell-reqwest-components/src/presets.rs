#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::{
        HttpMethodTypeProviderComponent, MethodArgExtractorComponent, UrlTypeProviderComponent,
    };
    use hypershell_components::dsl::{
        GetMethod, Header, PostMethod, SimpleHttpRequest, WithHeaders,
    };
    use reqwest::{Method, Url};

    use crate::components::RequestBuilderUpdaterComponent;
    use crate::providers::{
        ExtractReqwestMethod, HandleSimpleHttpRequest, UpdateRequestHeader, UpdateRequestHeaders,
    };

    cgp_preset! {
        HypershellReqwestPreset {
            HandlerComponent:
                ReqwestHandlerPreset::Provider,
            HttpMethodTypeProviderComponent:
                UseType<Method>,
            UrlTypeProviderComponent:
                UseType<Url>,
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
        RequestBuilderUpdaterPreset {
            <Args> WithHeaders<Args>:
                UpdateRequestHeaders,
            <Key, Value> Header<Key, Value>:
                UpdateRequestHeader,
        }
    }
}
