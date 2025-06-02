use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_type]
pub trait HasHttpMethodType {
    type HttpMethod;
}

#[cgp_component {
    provider: MethodArgExtractor,
    use_delegate: Arg,
}]
pub trait CanExtractMethodArg<Arg>: HasHttpMethodType {
    fn extract_method_arg(&self, _phantom: PhantomData<Arg>) -> Self::HttpMethod;
}
