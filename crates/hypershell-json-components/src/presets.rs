#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::extra::handler::HandlerComponent;
    use cgp::prelude::*;
    use hypershell_components::dsl::BytesToJson;

    use crate::providers::HandleDecodeJson;

    cgp_preset! {
        HypershellJsonPreset {
            HandlerComponent:
                JsonHandlerPreset::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        JsonHandlerPreset {
            <Value> BytesToJson<Value>:
                HandleDecodeJson,
        }
    }
}
