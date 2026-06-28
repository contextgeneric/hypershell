use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;
use hypershell_components::dsl::{BytesToStream, WebSocket};
use hypershell_components::providers::Call;
use hypershell_tokio_components::providers::{FuturesToTokioAsyncRead, WrapFuturesAsyncRead};
use hypershell_tokio_components::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

use crate::providers::HandleWebsocket;

delegate_components! {
    new HypershellTungsteniteProvider {
        open {HandlerComponent};

        @HandlerComponent
            .<Url, Params> WebSocket<Url, Params>
            .<S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,

        @HandlerComponent
            .<Url, Params> WebSocket<Url, Params>
            .<S> TokioAsyncReadStream<S>:
                PipeHandlers<Product![
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,

        @HandlerComponent
            .<Url, Params> WebSocket<Url, Params>
            .Vec<u8>:
                PipeHandlers<Product![
                    Call<BytesToStream>,
                    HandleWebsocket,
                    WrapFuturesAsyncRead,
                ]>,
    }
}
