use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, OutCode, InCode, Input> Handler<Context, OutCode, Input> for Call<InCode>
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
