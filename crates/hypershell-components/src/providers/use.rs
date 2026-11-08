use core::marker::PhantomData;

use cgp::extra::handler::{Handler, HandlerComponent};
use cgp::prelude::*;

use crate::dsl::Use;

/// Runs the named provider inline instead of the context's wiring for the stage. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new HandleUseProvider)]
impl<Context, Provider, Code, Input> Handler<Use<Provider, Code>, Input> for Context
where
    Context: HasErrorType,
    Provider: Handler<Context, Code, Input>,
{
    type Output = Provider::Output;

    async fn handle(
        context: &Context,
        _tag: PhantomData<Use<Provider, Code>>,
        input: Input,
    ) -> Result<Provider::Output, Context::Error> {
        Provider::handle(context, PhantomData, input).await
    }
}
