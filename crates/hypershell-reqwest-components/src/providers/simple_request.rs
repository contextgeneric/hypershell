use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::SimpleHttpRequest;
use reqwest::Response;

use crate::dsl::CoreHttpRequest;

#[derive(Debug)]
pub struct ErrorResponse {
    pub response: Response,
}

#[cgp_new_provider]
impl<Context, MethodArg, UrlArg, Headers, Input>
    Handler<Context, SimpleHttpRequest<MethodArg, UrlArg, Headers>, Input>
    for HandleSimpleHttpRequest
where
    Context: CanHandle<CoreHttpRequest<MethodArg, UrlArg, Headers>, Input, Output = Response>
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<ErrorResponse>,
    MethodArg: Send,
    UrlArg: Send,
    Headers: Send,
    Input: Send,
{
    type Output = Vec<u8>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleHttpRequest<MethodArg, UrlArg, Headers>>,
        body: Input,
    ) -> Result<Vec<u8>, Context::Error> {
        let response = context
            .handle(
                PhantomData::<CoreHttpRequest<MethodArg, UrlArg, Headers>>,
                body,
            )
            .await?;

        let status_code = response.status();

        if !status_code.is_success() {
            return Err(Context::raise_error(ErrorResponse { response }));
        }

        let response_body = response.bytes().await.map_err(Context::raise_error)?;

        Ok(response_body.into())
    }
}
