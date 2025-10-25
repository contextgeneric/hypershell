use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_impl(new ReturnInput)]
impl<Context, Code, Input> Handler<Code, Input> for Context
where
    Context: HasErrorType,
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
