#[cgp::re_export_imports]
mod preset {
    use std::vec::Vec;

    use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
    use cgp::prelude::*;
    use hypershell_components::dsl::{BytesToStream, WebSocket};
    use hypershell_components::providers::Call;
    use hypershell_tokio_components::providers::{FuturesToTokioAsyncRead, WrapFuturesAsyncRead};
    use hypershell_tokio_components::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

    use crate::providers::HandleWebsocket;

    cgp_preset! {
        #[wrap_provider(UseDelegate)]
        TungsteniteHandlerPreset {
            <Url, Params> WebSocket<Url, Params>:
                WebSocketHandlers::Provider,
        }
    }

    cgp_preset! {
        #[wrap_provider(UseInputDelegate)]
        WebSocketHandlers {
            <S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,
            <S> TokioAsyncReadStream<S>:
                PipeHandlers<Product![
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,
            Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,
        }
    }
}
