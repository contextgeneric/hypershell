use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::{AsyncRead, AsyncReadExt};

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleStreamToBytes
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Code: Send,
    Input: Send + AsyncRead + Unpin,
{
    type Output = Vec<u8>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        mut input: Input,
    ) -> Result<Vec<u8>, Context::Error> {
        let mut output = Vec::new();
        input
            .read_to_end(&mut output)
            .await
            .map_err(Context::raise_error)?;

        Ok(output)
    }
}
