use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for ReturnInput
where
    Context: HasAsyncErrorType,
    Code: Send,
    Input: Send,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Input, Context::Error> {
        Ok(input)
    }
}
