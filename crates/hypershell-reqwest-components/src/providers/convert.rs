use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use reqwest::Body;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for StreamToBody
where
    Context: HasErrorType,
    Input: Send + AsyncRead + 'static,
{
    type Output = Body;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Body, Context::Error> {
        let stream = ReaderStream::new(input);
        let body = Body::wrap_stream(stream);
        Ok(body)
    }
}
