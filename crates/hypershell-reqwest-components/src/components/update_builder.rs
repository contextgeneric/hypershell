use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use reqwest::RequestBuilder;

#[cgp_component(RequestBuilderUpdater)]
#[derive_delegate(UseDelegate<Args>)]
#[prefix(@hypershell.reqwest in DefaultNamespace)]
pub trait CanUpdateRequestBuilder<Args>: HasErrorType {
    fn update_request_builder(
        &self,
        _phantom: PhantomData<Args>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Self::Error>;
}
