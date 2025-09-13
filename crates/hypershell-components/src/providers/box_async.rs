use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_new_provider]
impl<Context, Code, Input, InHandler> Handler<Context, Code, Input> for BoxHandler<InHandler>
where
    Context: HasErrorType,
    InHandler: 'static + Handler<Context, Code, Input>,
    Code: 'static,
    Input: 'static,
{
    type Output = InHandler::Output;

    fn handle(
        context: &Context,
        code: PhantomData<Code>,
        input: Input,
    ) -> impl Future<Output = Result<Self::Output, Context::Error>> {
        let future: Pin<Box<dyn Future<Output = Result<Self::Output, Context::Error>>>> =
            Box::pin(InHandler::handle(context, code, input));

        future
    }
}
