#[cgp::re_export_imports]
mod preset {
    use cgp::prelude::*;
    use hypershell_components::dsl::WebSocket;

    use crate::providers::HandleWebsocket;

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        TungsteniteHandlerPreset {
            <Url, Params> WebSocket<Url, Params>:
                HandleWebsocket,
        }
    }
}
