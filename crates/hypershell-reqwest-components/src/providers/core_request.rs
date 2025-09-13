use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::components::{CanExtractMethodArg, CanExtractUrlArg};
use reqwest::{Body, Method, Response, Url};

use crate::components::{CanUpdateRequestBuilder, HasReqwestClient};
use crate::dsl::CoreHttpRequest;

#[cgp_new_provider]
impl<Context, MethodArg, UrlArg, Headers, Input>
    Handler<Context, CoreHttpRequest<MethodArg, UrlArg, Headers>, Input> for HandleCoreHttpRequest
where
    Context: HasReqwestClient
        + CanExtractUrlArg<UrlArg, Url = Url>
        + CanExtractMethodArg<MethodArg, HttpMethod = Method>
        + CanUpdateRequestBuilder<Headers>
        + CanRaiseError<reqwest::Error>,
    Input: Into<Body>,
{
    type Output = Response;

    async fn handle(
        context: &Context,
        _tag: PhantomData<CoreHttpRequest<MethodArg, UrlArg, Headers>>,
        body: Input,
    ) -> Result<Response, Context::Error> {
        let client = context.request_client();
        let url = context.extract_url_arg(PhantomData)?;
        let method = context.extract_method_arg(PhantomData);

        let builder = client.request(method, url);

        let builder = context
            .update_request_builder(PhantomData, builder)?
            .body(body);

        let response = builder.send().await.map_err(Context::raise_error)?;

        Ok(response)
    }
}
