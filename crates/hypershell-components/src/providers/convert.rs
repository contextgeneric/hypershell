use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::str::Utf8Error;

use cgp::extra::handler::{Computer, ComputerComponent, Handler, HandlerComponent};
use cgp::prelude::*;

use crate::dsl::ConvertTo;

#[cgp_new_provider]
impl<Context, Input, Output> Computer<Context, ConvertTo<Output>, Input> for HandleConvert
where
    Input: Into<Output>,
{
    type Output = Output;

    fn compute(_context: &Context, _tag: PhantomData<ConvertTo<Output>>, input: Input) -> Output {
        input.into()
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for DecodeUtf8Bytes
where
    Context: CanRaiseError<Utf8Error> + for<'a> CanWrapError<DecodeUtf8InputError<'a>>,
    Input: AsRef<[u8]>,
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
