use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use reqwest::RequestBuilder;

#[cgp_component(RequestBuilderUpdater)]
pub trait CanUpdateRequestBuilder<Args>: HasErrorType {
    fn update_request_builder(
        &self,
        _phantom: PhantomData<Args>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Self::Error>;
}

#[cgp_provider]
impl<Context, Args, Components, Delegate> RequestBuilderUpdater<Context, Args>
    for UseDelegate<Components>
where
    Context: HasErrorType,
    Components: DelegateComponent<Args, Delegate = Delegate>,
    Delegate: RequestBuilderUpdater<Context, Args>,
{
    fn update_request_builder(
        context: &Context,
        phantom: PhantomData<Args>,
        builder: RequestBuilder,
    ) -> Result<RequestBuilder, Context::Error> {
        Delegate::update_request_builder(context, phantom, builder)
    }
}
