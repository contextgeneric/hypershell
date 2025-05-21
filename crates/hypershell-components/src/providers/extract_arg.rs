use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::components::{ArgExtractor, ArgExtractorComponent};
use crate::dsl::StaticArg;

#[cgp_new_provider]
impl<Context, Arg> ArgExtractor<Context, StaticArg<Arg>> for ExtractStaticArg
where
    Arg: Default + Display,
{
    fn extract_arg(_context: &Context, _phantom: PhantomData<StaticArg<Arg>>) -> String {
        Arg::default().to_string()
    }
}

#[cgp_new_provider]
impl<Context, Tag> ArgExtractor<Context, UseField<Tag>> for ExtractFieldArg
where
    Context: HasField<Tag, Value: Display>,
{
    fn extract_arg(context: &Context, _phantom: PhantomData<UseField<Tag>>) -> String {
        context.get_field(PhantomData).to_string()
    }
}
