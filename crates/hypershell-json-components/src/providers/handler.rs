use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::DecodeJson;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cgp_new_provider]
impl<Context, Input, Output> Handler<Context, DecodeJson<Output>, Input> for HandleDecodeJson
where
    Context: CanRaiseError<serde_json::Error>,
    Input: AsRef<[u8]>,
    Output: DeserializeOwned,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<DecodeJson<Output>>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let output = serde_json::from_slice(input.as_ref()).map_err(Context::raise_error)?;
        Ok(output)
    }
}

#[cgp_new_provider]
impl<Context, Code, Input> Handler<Context, Code, Input> for HandleEncodeJson
where
    Context: CanRaiseError<serde_json::Error>,
    Input: Serialize,
{
    type Output = Vec<u8>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        input: Input,
    ) -> Result<Vec<u8>, Context::Error> {
        let output = serde_json::to_vec(&input).map_err(Context::raise_error)?;
        Ok(output)
    }
}
