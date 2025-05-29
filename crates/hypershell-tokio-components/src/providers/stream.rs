use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::io::Cursor;
use futures::{AsyncRead, AsyncReadExt};
use hypershell_components::dsl::{BytesToStream, StreamToBytes, StreamToString};

pub struct ConvertStream;

#[cgp_provider]
impl<Context, Input> Handler<Context, StreamToBytes, Input> for ConvertStream
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsyncRead + Unpin,
{
    type Output = Vec<u8>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<StreamToBytes>,
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

#[cgp_provider]
impl<Context, Input> Handler<Context, StreamToString, Input> for ConvertStream
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsyncRead + Unpin,
{
    type Output = String;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<StreamToString>,
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

#[cgp_provider]
impl<Context, Input> Handler<Context, BytesToStream, Input> for ConvertStream
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + AsRef<[u8]> + Unpin,
{
    type Output = Cursor<Input>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<BytesToStream>,
        input: Input,
    ) -> Result<Cursor<Input>, Context::Error> {
        Ok(Cursor::new(input))
    }
}
