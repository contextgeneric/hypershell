use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::CanExtractStringArg;
use hypershell_components::dsl::Header;
use reqwest::RequestBuilder;
use reqwest::header::{HeaderName, HeaderValue, InvalidHeaderName, InvalidHeaderValue};

use crate::components::{RequestBuilderUpdater, RequestBuilderUpdaterComponent};

#[cgp_new_provider]
impl<Context, Key, Value> RequestBuilderUpdater<Context, Header<Key, Value>> for UpdateRequestHeader
where
    Context: HasErrorType
        + CanExtractStringArg<Key>
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
