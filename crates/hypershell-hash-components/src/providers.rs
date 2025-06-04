use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::{TryStream, TryStreamExt};
use sha2::Digest;
use sha2::digest::generic_array::GenericArray;

#[cgp_new_provider]
impl<Context, Code, Input, Hasher> Handler<Context, Code, Input> for HandleStreamChecksum<Hasher>
where
    Context: CanRaiseAsyncError<Input::Error>,
    Code: Send,
    Input: Send + Unpin + TryStream,
    Hasher: Send + Digest,
    Input::Ok: AsRef<[u8]>,
{
    type Output = GenericArray<u8, Hasher::OutputSize>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Code>,
        mut input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let mut hasher = Hasher::new();

        while let Some(bytes) = input.try_next().await.map_err(Context::raise_error)? {
            hasher.update(bytes);
        }

        Ok(hasher.finalize())
    }
}
