use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell::prelude::CanHandle;

use crate::dsl::Compare;

#[cgp_new_provider]
impl<Context, CodeA, CodeB, InputA, InputB, Output>
    Handler<Context, Compare<CodeA, CodeB>, (InputA, InputB)> for HandleCompare
where
    Context: CanHandle<CodeA, InputA, Output = Output> + CanHandle<CodeB, InputB, Output = Output>,
    Output: Eq,
{
    type Output = bool;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Compare<CodeA, CodeB>>,
        (input_a, input_b): (InputA, InputB),
    ) -> Result<bool, Context::Error> {
        let futures_a = context.handle(PhantomData::<CodeA>, input_a);
        let futures_b = context.handle(PhantomData::<CodeB>, input_b);

        let (res_a, res_b) = futures::try_join!(futures_a, futures_b)?;

        Ok(res_a == res_b)
    }
}
