use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

/// Encodes byte-like input as a lowercase hexadecimal string. See
/// [the native checksum example](https://contextgeneric.dev/docs/projects/hypershell/examples/http-checksum-native).
#[cgp_impl(new HandleBytesToHex)]
impl<Context, Code, Input> Handler<Code, Input> for Context
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
