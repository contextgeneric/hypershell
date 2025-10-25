use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;
use futures::{TryStream, TryStreamExt};
use sha2::Digest;
use sha2::digest::generic_array::GenericArray;

use crate::dsl::Checksum;

#[cgp_impl(new HandleStreamChecksum)]
impl<Context, Input, Hasher> Handler<Checksum<Hasher>, Input> for Context
where
    Context: CanRaiseError<Input::Error>,
    Input: Unpin + TryStream,
    Hasher: Digest,
    Input::Ok: AsRef<[u8]>,
{
    type Output = GenericArray<u8, Hasher::OutputSize>;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Checksum<Hasher>>,
        mut input: Input,
    ) -> Result<Self::Output, Context::Error> {
        let mut hasher = Hasher::new();

        while let Some(bytes) = input.try_next().await.map_err(Context::raise_error)? {
            hasher.update(bytes);
        }

        Ok(hasher.finalize())
    }
}
