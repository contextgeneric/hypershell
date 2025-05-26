use core::marker::PhantomData;

use cgp::prelude::*;
use reqwest::RequestBuilder;

#[cgp_component(RequestBuilderUpdater)]
pub trait CanUpdateRequestBuilder<Args>: HasErrorType {
    fn update_request_builder(
        &self,
        _phantom: PhantomData<Args>,
        builder: &mut RequestBuilder,
    ) -> Result<(), Self::Error>;
}
