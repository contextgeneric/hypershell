use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_impl(new Call<InCode>)]
impl<Context, OutCode, InCode, Input> Handler<OutCode, Input> for Context
where
    Context: CanHandle<InCode, Input>,
{
    type Output = Context::Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<OutCode>,
        input: Input,
    ) -> Result<Context::Output, Context::Error> {
        context.handle(PhantomData, input).await
    }
}
