use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code: Send, Input: Send, InHandler> Handler<Context, Code, Input>
    for BoxHandler<InHandler>
where
    Context: HasAsyncErrorType,
    InHandler: Send + 'static + Handler<Context, Code, Input>,
    Code: Send + 'static,
    Input: Send + 'static,
{
    type Output = InHandler::Output;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> impl Future<Output = Result<Self::Output, Context::Error>> + Send {
        let future: Pin<Box<dyn Future<Output = Result<Self::Output, Context::Error>> + Send>> =
            Box::pin(InHandler::handle(context, code, input));

        future
    }
}
