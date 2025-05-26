use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::str::Utf8Error;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for DecodeUtf8Bytes
where
    Context: CanRaiseAsyncError<Utf8Error> + for<'a> CanWrapAsyncError<DecodeUtf8InputError<'a>>,
    Code: Send,
    Input: Send + AsRef<[u8]>,
{
    type Output = String;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<String, Context::Error> {
        let raw_input = input.as_ref();

        let parsed = str::from_utf8(raw_input)
            .map_err(Context::raise_error)
            .map_err(|e| Context::wrap_error(e, DecodeUtf8InputError { raw_input }))?;

        Ok(parsed.to_owned())
    }
}

pub struct DecodeUtf8InputError<'a> {
    pub raw_input: &'a [u8],
}

impl<'a> Debug for DecodeUtf8InputError<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "failed to decode input bytes as UTF-8 string")
    }
}
