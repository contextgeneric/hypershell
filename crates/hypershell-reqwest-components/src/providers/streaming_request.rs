use core::marker::PhantomData;
use core::pin::Pin;
use std::io::ErrorKind;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use futures::TryStreamExt;
use hypershell_components::dsl::StreamingHttpRequest;
use reqwest::Response;
use tokio::io::AsyncRead;
use tokio_util::compat::FuturesAsyncReadCompatExt;

use crate::dsl::CoreHttpRequest;
use crate::providers::ErrorResponse;

#[cgp_new_provider]
impl<Context, MethodArg, UrlArg, Headers, Input>
    Handler<Context, StreamingHttpRequest<MethodArg, UrlArg, Headers>, Input>
    for HandleStreamingHttpRequest
where
    Context: CanHandle<CoreHttpRequest<MethodArg, UrlArg, Headers>, Input, Output = Response>
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<ErrorResponse>,
    MethodArg: Send,
    UrlArg: Send,
    Headers: Send,
    Input: Send,
{
    type Output = Pin<Box<dyn AsyncRead + Send>>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<StreamingHttpRequest<MethodArg, UrlArg, Headers>>,
        body: Input,
    ) -> Result<Pin<Box<dyn AsyncRead + Send>>, Context::Error> {
        let response = context.handle(PhantomData, body).await?;

        let status_code = response.status();

        if !status_code.is_success() {
            return Err(Context::raise_error(ErrorResponse { response }));
        }

        let response_stream = response
            .bytes_stream()
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            .into_async_read();

        Ok(Box::pin(response_stream.compat()))
    }
}
