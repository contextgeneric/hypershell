use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

/// Returns its input unchanged, for a stage that only passes a value on. See
/// [the streams and input dispatch page](https://contextgeneric.dev/docs/projects/hypershell/architecture/streams-and-input-dispatch).
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
