use alloc::borrow::ToOwned;
use alloc::string::String;
use core::fmt::Debug;
use core::marker::PhantomData;
use core::str::Utf8Error;

use cgp::extra::handler::{Computer, ComputerComponent, Handler, HandlerComponent};
use cgp::prelude::*;

use crate::dsl::ConvertTo;

#[cgp_impl(new HandleConvert)]
impl<Input, Output> Computer<ConvertTo<Output>, Input>
where
    Input: Into<Output>,
{
    type Output = Output;

    fn compute(&self, _tag: PhantomData<ConvertTo<Output>>, input: Input) -> Output {
        input.into()
    }
}

#[cgp_impl(new DecodeUtf8Bytes)]
#[use_type(HasErrorType::Error)]
impl<Code, Input> Handler<Code, Input>
where
    Self: CanRaiseError<Utf8Error> + for<'a> CanWrapError<DecodeUtf8InputError<'a>>,
    Input: AsRef<[u8]>,
{
    type Output = String;

    async fn handle(&self, _tag: PhantomData<Code>, input: Input) -> Result<String, Error> {
        let raw_input = input.as_ref();

        let parsed = str::from_utf8(raw_input)
            .map_err(Self::raise_error)
            .map_err(|e| Self::wrap_error(e, DecodeUtf8InputError { raw_input }))?;

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
