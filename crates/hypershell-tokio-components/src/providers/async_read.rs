use cgp::prelude::{HandlerComponent, delegate_components};
use hypershell_components::providers::ReturnInput;

use crate::providers::{FuturesToTokioAsyncRead, HandleBytesToTokioAsyncRead};
use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

delegate_components! {
    new HandleToTokioAsyncRead {
        open HandlerComponent;

        @HandlerComponent
            .<Code> Code
            .<S> FuturesAsyncReadStream<S>:
                FuturesToTokioAsyncRead,
        @HandlerComponent
            .<Code> Code
            .<S> TokioAsyncReadStream<S>:
                ReturnInput,
        @HandlerComponent
            .<Code> Code.[
                Vec<u8>,
                String,
            ]:
                HandleBytesToTokioAsyncRead,
    }
}
