use core::marker::PhantomData;

use cgp::extra::handler::{CanHandle, Handler, HandlerComponent};
use cgp::prelude::*;

use crate::dsl::Pipe;

pub struct RunPipe;

#[cgp_provider]
impl<Context, Input, Intermediate, Output, CurrentHandler, RestHandlers>
    Handler<Context, Pipe<Cons<CurrentHandler, RestHandlers>>, Input> for RunPipe
where
    Context: CanHandle<CurrentHandler, Input, Output = Intermediate>,
    Self: Handler<Context, Pipe<RestHandlers>, Intermediate, Output = Output>,
    Input: Send,
    Output: Send,
    CurrentHandler: Send,
    RestHandlers: Send,
    Intermediate: Send,
{
    type Output = Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Pipe<Cons<CurrentHandler, RestHandlers>>>,
        input: Input,
    ) -> Result<Output, Context::Error> {
        let intermediate = context.handle(PhantomData, input).await?;
        Self::handle(context, PhantomData, intermediate.into()).await
    }
}

#[cgp_provider]
impl<Context, Input> Handler<Context, Pipe<Nil>, Input> for RunPipe
where
    Context: HasAsyncErrorType,
    Input: Send,
{
    type Output = Input;

    async fn handle(
        _context: &Context,
        _tag: PhantomData<Pipe<Nil>>,
        input: Input,
    ) -> Result<Input, Context::Error> {
        Ok(input)
    }
}
