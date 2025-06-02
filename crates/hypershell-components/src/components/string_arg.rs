use alloc::borrow::Cow;
use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
    provider: StringArgExtractor,
    use_delegate: Arg,
}]
pub trait CanExtractStringArg<Arg> {
    fn extract_string_arg(&self, _phantom: PhantomData<Arg>) -> Cow<'_, str>;
}
