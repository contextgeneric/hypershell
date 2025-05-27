#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::{cgp_preset, *};
    use hypershell_components::components::MethodArgExtractorComponent;
    use hypershell_components::dsl::{GetMethod, Header, PostMethod, WithHeaders};

    use crate::components::RequestBuilderUpdaterComponent;
    use crate::providers::{ExtractReqwestMethod, UpdateRequestHeader, UpdateRequestHeaders};

    cgp_preset! {
        HypershellReqwestPreset {
            MethodArgExtractorComponent:
                MethodArgExtractorPreset::Provider,
            RequestBuilderUpdaterComponent:
                RequestBuilderUpdaterPreset::Provider,
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
