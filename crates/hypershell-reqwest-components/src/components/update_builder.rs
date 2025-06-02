use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use reqwest::RequestBuilder;

#[cgp_component {
    provider: RequestBuilderUpdater,
    derive_delegate: UseDelegate<Args>,
}]
pub trait CanUpdateRequestBuilder<Args>: HasErrorType {
    fn update_request_builder(
        &self,
        _phantom: PhantomData<Args>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Self::Error>;
}
