use core::convert::Infallible;
use core::iter::Once;
use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::AsyncRead as FuturesAsyncRead;
use futures::io::Cursor;
use futures::stream::Iter;
use tokio::io::{AsyncRead as TokioAsyncRead, AsyncReadExt as _};
use tokio_util::compat::{Compat, FuturesAsyncReadCompatExt, TokioAsyncReadCompatExt};
use tokio_util::io::ReaderStream;

use crate::types::{FuturesAsyncReadStream, FuturesStream, TokioAsyncReadStream};

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleTokioAsyncReadToBytes
where
    Context: CanRaiseError<std::io::Error>,
    Input: TokioAsyncRead + Unpin,
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
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleTokioAsyncReadToString
where
    Context: CanRaiseError<std::io::Error>,
    Input: TokioAsyncRead + Unpin,
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
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleBytesToTokioAsyncRead
where
    Context: CanRaiseError<std::io::Error>,
    Input: AsRef<[u8]> + Unpin,
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
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleBytesToStream
where
    Context: CanRaiseError<std::io::Error>,
    Input: AsRef<[u8]> + Unpin,
{
    type Output = FuturesStream<Iter<Once<Result<Input, Infallible>>>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        Ok(futures::stream::iter(core::iter::once(Ok(input))).into())
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for FuturesToTokioAsyncRead
where
    Context: HasErrorType,
    Input: FuturesAsyncRead + Unpin,
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
impl<Context, Code, Input> Handler<Context, Code, Input> for TokioToFuturesAsyncRead
where
    Context: HasErrorType,
    Input: TokioAsyncRead + Unpin,
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

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for WrapTokioAsyncRead
where
    Context: HasErrorType,
    Input: TokioAsyncRead + Unpin,
{
    type Output = TokioAsyncReadStream<Input>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<TokioAsyncReadStream<Input>, Context::Error> {
        Ok(input.into())
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for WrapFuturesAsyncRead
where
    Context: HasErrorType,
    Input: FuturesAsyncRead + Unpin,
{
    type Output = FuturesAsyncReadStream<Input>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<FuturesAsyncReadStream<Input>, Context::Error> {
        Ok(input.into())
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for AsyncReadToStream
where
    Context: HasErrorType,
    Input: TokioAsyncRead + Unpin,
{
    type Output = FuturesStream<ReaderStream<Input>>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<FuturesStream<ReaderStream<Input>>, Context::Error> {
        Ok(ReaderStream::new(input).into())
    }
}
