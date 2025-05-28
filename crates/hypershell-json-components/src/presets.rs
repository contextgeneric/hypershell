#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::dsl::{DecodeJson, EncodeJson};

    use crate::providers::{HandleDecodeJson, HandleEncodeJson};

    cgp_preset! {
        HypershellJsonPreset {
            HandlerComponent:
                JsonHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        JsonHandlerPreset {
            <Value> DecodeJson<Value>:
                HandleDecodeJson,
            EncodeJson:
                HandleEncodeJson,
        }
    }
}
