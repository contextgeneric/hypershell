use core::marker::PhantomData;

use cgp::prelude::*;

use crate::components::{HasHttpMethodType, MethodArgExtractor, MethodArgExtractorComponent};
use crate::dsl::FieldArg;

#[cgp_new_provider]
impl<Context, Tag> MethodArgExtractor<Context, FieldArg<Tag>> for ExtractMethodFieldArg
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
