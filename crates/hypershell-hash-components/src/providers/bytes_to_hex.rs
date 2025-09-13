use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleBytesToHex
where
    Context: HasErrorType,
    Input: AsRef<[u8]>,
{
    type Output = String;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<String, Context::Error> {
        let output = hex::encode(input);
        Ok(output)
    }
}
