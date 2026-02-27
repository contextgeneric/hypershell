use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_impl(new Call<InCode>)]
impl<OutCode, InCode, Input, Output> Handler<OutCode, Input>
where
    Self: CanHandle<InCode, Input, Output = Output>,
{
    type Output = Output;

    async fn handle(
        &self,
        _tag: PhantomData<OutCode>,
        input: Input,
    ) -> Result<Self::Output, Self::Error> {
        self.handle(PhantomData, input).await
    }
}
