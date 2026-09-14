use cgp::extra::handler::PipeHandlers;
use cgp::prelude::*;

use crate::providers::{AsyncReadToStream, FuturesToTokioAsyncRead, HandleBytesToStream};
use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

delegate_components! {
    new HandleToFuturesStream {
        open HandlerComponent;

        @HandlerComponent
            .<Code> Code
            .<S> FuturesAsyncReadStream<S>:
                PipeHandlers<Product![
                    FuturesToTokioAsyncRead,
                    AsyncReadToStream,
                ]>,
        @HandlerComponent
            .<Code> Code
            .<S> TokioAsyncReadStream<S>:
                AsyncReadToStream,
        @HandlerComponent
            .<Code> Code
            .[
                Vec<u8>,
                String,
            ]:
                HandleBytesToStream,
    }
}
