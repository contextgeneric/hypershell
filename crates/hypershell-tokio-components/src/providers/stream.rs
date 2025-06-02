use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::io::Cursor;
use futures::{AsyncRead, AsyncReadExt};

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ConvertStreamToBytes
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsyncRead + Unpin,
    Code: Send,
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

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ConvertStreamToString
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsyncRead + Unpin,
    Code: Send,
{
    type Output = String;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        mut input: Input,
    ) -> Result<String, Context::Error> {
        let mut output = String::new();

        input
            .read_to_string(&mut output)
            .await
            .map_err(Context::raise_error)?;

        Ok(output)
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ConvertBytesToStream
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsRef<[u8]> + Unpin + 'static,
    Code: Send,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        Ok(Box::pin(Cursor::new(input)))
    }
}
