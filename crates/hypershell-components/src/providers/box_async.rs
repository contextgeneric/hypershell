use alloc::boxed::Box;
use core::marker::PhantomData;
use core::pin::Pin;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

#[cgp_impl(new BoxHandler<InHandler>)]
#[use_type(HasErrorType::Error)]
#[use_provider(InHandler: Handler<Code, Input>)]
impl<Code, Input, InHandler> Handler<Code, Input>
where
    InHandler: 'static,
    Code: 'static,
    Input: 'static,
{
    type Output = InHandler::Output;

    fn handle(
        &self,
        code: PhantomData<Code>,
        input: Input,
    ) -> impl Future<Output = Result<Self::Output, Error>> {
        let future: Pin<Box<dyn Future<Output = Result<Self::Output, Error>>>> =
            Box::pin(InHandler::handle(self, code, input));

        future
    }
}
