use cgp::extra::handler::UseInputDelegate;
use cgp::prelude::{HandlerComponent, delegate_components};
use hypershell_components::providers::ReturnInput;

use crate::providers::{FuturesToTokioAsyncRead, HandleBytesToTokioAsyncRead};
use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

delegate_components! {
    new HandleToTokioAsyncRead {
        HandlerComponent:
            UseInputDelegate<ToTokioAsyncReadHandlers>,
    }
}

delegate_components! {
    new ToTokioAsyncReadHandlers {
        <S> FuturesAsyncReadStream<S>:
            FuturesToTokioAsyncRead,
        <S> TokioAsyncReadStream<S>:
            ReturnInput,
        [
            Vec<u8>,
            String,
        ]:
            HandleBytesToTokioAsyncRead,
    }
}
