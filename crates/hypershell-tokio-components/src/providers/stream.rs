use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::AsyncRead;
use futures::io::Cursor;
use tokio::io::{AsyncRead as TokioAsyncRead, AsyncReadExt as _};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};

use crate::types::{FuturesAsyncReadStream, TokioAsyncReadStream};

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ConvertStreamToBytes
where
    Context: CanRaiseAsyncError<std::io::Error>,
    Input: Send + TokioAsyncRead + Unpin,
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
    Input: Send + TokioAsyncRead + Unpin,
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
    type Output = TokioAsyncReadStream<Compat<Cursor<Input>>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<TokioAsyncReadStream<Compat<Cursor<Input>>>, Context::Error> {
        Ok(Cursor::new(input).compat().into())
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for FuturesToTokioStream
where
    Context: HasAsyncErrorType,
    Input: Send + AsyncRead + Unpin + 'static,
    Code: Send,
{
    type Output = TokioAsyncReadStream<Compat<Input>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<TokioAsyncReadStream<Compat<Input>>, Context::Error> {
        Ok(input.compat().into())
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for TokioToFuturesStream
where
    Context: HasAsyncErrorType,
    Input: Send + TokioAsyncRead + Unpin + 'static,
    Code: Send,
{
    type Output = FuturesAsyncReadStream<Compat<Input>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<FuturesAsyncReadStream<Compat<Input>>, Context::Error> {
        Ok(input.compat().into())
    }
}
