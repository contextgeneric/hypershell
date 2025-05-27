use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::CanExtractStringArg;
use hypershell_components::dsl::{Header, WithHeaders};
use reqwest::RequestBuilder;
use reqwest::header::{HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue};

use crate::components::{
    CanUpdateRequestBuilder, RequestBuilderUpdater, RequestBuilderUpdaterComponent,
};

#[cgp_new_provider]
impl<Context, Key, Value> RequestBuilderUpdater<Context, Header<Key, Value>> for UpdateRequestHeader
where
    Context: CanExtractStringArg<Key>
        + CanExtractStringArg<Value>
        + CanRaiseError<InvalidHeaderName>
        + CanRaiseError<InvalidHeaderValue>,
{
    fn update_request_builder(
        context: &Context,
        _phantom: PhantomData<Header<Key, Value>>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Context::Error> {
        let key_str = context.extract_string_arg(PhantomData::<Key>);
        let value_str = context.extract_string_arg(PhantomData::<Value>);

        let key = HeaderName::from_bytes(key_str.as_bytes()).map_err(Context::raise_error)?;

        let value = HeaderValue::from_bytes(value_str.as_bytes()).map_err(Context::raise_error)?;

        Ok(builder.header(key, value))
    }
}

pub struct UpdateRequestHeaders;

#[cgp_provider]
impl<Context, Arg, Args> RequestBuilderUpdater<Context, WithHeaders<Cons<Arg, Args>>>
    for UpdateRequestHeaders
where
    Context: CanUpdateRequestBuilder<Arg>,
    Self: RequestBuilderUpdater<Context, WithHeaders<Args>>,
{
    fn update_request_builder(
        context: &Context,
        _phantom: PhantomData<WithHeaders<Cons<Arg, Args>>>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Context::Error> {
        let builder = context.update_request_builder(PhantomData, builder)?;
        let builder =
            Self::update_request_builder(context, PhantomData::<WithHeaders<Args>>, builder)?;

        Ok(builder)
    }
}

#[cgp_provider]
impl<Context> RequestBuilderUpdater<Context, WithHeaders<Nil>> for UpdateRequestHeaders
where
    Context: HasErrorType,
{
    fn update_request_builder(
        _context: &Context,
        _phantom: PhantomData<WithHeaders<Nil>>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Context::Error> {
        Ok(builder)
    }
}
