use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

use crate::dsl::Use;

pub struct HandleUseProvider;

#[cgp_provider]
impl<Context, Provider, Code, Input> Handler<Context, Use<Provider, Code>, Input>
    for HandleUseProvider
where
    Context: HasErrorType,
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Use<Provider, Code>>,
        input: Input,
    ) -> Result<Provider::Output, Context::Error> {
        Provider::handle(context, PhantomData, input).await
    }
}
