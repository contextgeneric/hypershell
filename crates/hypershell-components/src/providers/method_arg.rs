use core::marker::PhantomData;

use cgp::core::field::traits::HasField;
use cgp::prelude::*;

use crate::components::{HasHttpMethodType, MethodArgExtractor, MethodArgExtractorComponent};
use crate::dsl::FieldArg;

/// Reads the HTTP method from the context field named `Tag`. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
#[cgp_impl(new ExtractMethodFieldArg)]
impl<Context, Tag> MethodArgExtractor<FieldArg<Tag>> for Context
where
    Context: HasHttpMethodType + HasField<Tag, Value = Context::HttpMethod>,
    Context::HttpMethod: Clone,
{
    fn extract_method_arg(
        context: &Context,
        _phantom: PhantomData<FieldArg<Tag>>,
    ) -> Context::HttpMethod {
        context.get_field(PhantomData).clone()
    }
}
