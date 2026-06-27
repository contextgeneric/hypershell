use cgp::extra::handler::{PipeHandlers, UseInputDelegate};
use cgp::prelude::*;

use crate::providers::{AsyncReadToStream, FuturesToTokioAsyncRead, HandleBytesToStream};
use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

delegate_components! {
    new HandleToFuturesStream {
        HandlerComponent:
            UseInputDelegate<ToFuturesStreamHandlers>,
    }
}

delegate_components! {
    new ToFuturesStreamHandlers {
        <S> FuturesAsyncReadStream<S>:
            PipeHandlers<Product![
                FuturesToTokioAsyncRead,
                AsyncReadToStream,
            ]>,
        <S> TokioAsyncReadStream<S>:
            AsyncReadToStream,
        [
            Vec<u8>,
            String,
        ]:
            HandleBytesToStream,
    }
}
