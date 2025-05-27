use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::{CanExtractMethodArg, CanExtractUrlArg};
use hypershell_components::dsl::SimpleHttpRequest;
use reqwest::{Body, Method, Response, Url};

use crate::components::{CanUpdateRequestBuilder, HasReqwestClient};

#[derive(Debug)]
pub struct ErrorResponse {
    pub response: Response,
}

#[cgp_new_provider]
impl<Context, MethodArg, UrlArg, Headers, Input>
    Handler<Context, SimpleHttpRequest<MethodArg, UrlArg, Headers>, Input>
    for HandleSimpleHttpRequest
where
    Context: HasReqwestClient
        + CanExtractUrlArg<UrlArg, Url = Url>
        + CanExtractMethodArg<MethodArg, HttpMethod = Method>
        + CanUpdateRequestBuilder<Headers>
        + CanRaiseAsyncError<reqwest::Error>
        + CanRaiseAsyncError<ErrorResponse>,
    MethodArg: Send,
    UrlArg: Send,
    Headers: Send,
    Input: Send + Into<Body>,
{
    type Output = Vec<u8>;

    async fn handle(
        context: &Context,
        _tag: PhantomData<SimpleHttpRequest<MethodArg, UrlArg, Headers>>,
        body: Input,
    ) -> Result<Vec<u8>, Context::Error> {
        let client = context.request_client();
        let url = context.extract_url_arg(PhantomData)?;
        let method = context.extract_method_arg(PhantomData);

        let builder = client.request(method, url);

        let builder = context
            .update_request_builder(PhantomData, builder)?
            .body(body);

        let response = builder.send().await.map_err(Context::raise_error)?;

        let status_code = response.status();

        if !status_code.is_success() {
            return Err(Context::raise_error(ErrorResponse { response }));
        }

        let response_body = response.bytes().await.map_err(Context::raise_error)?;

        Ok(response_body.into())
    }
}
