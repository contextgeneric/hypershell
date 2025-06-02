#[cgp::re_export_imports]
mod preset {
    use core::pin::Pin;
    use std::vec::Vec;

    use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
    use cgp::prelude::*;
    use futures::AsyncRead;
    use hypershell_components::dsl::{BytesToStream, WebSocket};
    use hypershell_components::providers::Call;
    use tokio::io::AsyncRead as TokioAsyncRead;

    use crate::providers::HandleWebsocket;

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        TungsteniteHandlerPreset {
            <Url, Params> WebSocket<Url, Params>:
                HandleWebsocket,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        WebSocketHandlers {
            Pin<Box<dyn AsyncRead + Send>>:
                HandleWebsocket,
            Pin<Box<dyn TokioAsyncRead + Send>>:
                HandleWebsocket,
            Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleWebsocket,
                ]>,
        }
    }
}
