use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use hypershell_components::dsl::BytesToJson;
use serde::de::DeserializeOwned;

#[cgp_new_provider]
impl<Context, Input, Output> Handler<Context, BytesToJson<Output>, Input> for HandleDecodeJson
where
    Context: CanRaiseAsyncError<serde_json::Error>,
    Input: Send + AsRef<[u8]>,
    Output: Send + DeserializeOwned,
{
    type Output = Output;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<BytesToJson<Output>>,
        input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let output = serde_json::from_slice(input.as_ref()).map_err(Context::raise_error)?;
        Ok(output)
    }
}
