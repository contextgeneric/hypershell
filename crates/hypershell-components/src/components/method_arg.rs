use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

/// A context's abstract HTTP-method type.
#[cgp_type]
#[prefix(@hypershell.core in DefaultNamespace)]
pub trait HasHttpMethodType {
    type HttpMethod;
}

/// Extracts an HTTP method from the context.
#[cgp_component(MethodArgExtractor)]
#[prefix(@hypershell.core in DefaultNamespace)]
#[derive_delegate(UseDelegate<Arg>)]
pub trait CanExtractMethodArg<Arg>: HasHttpMethodType {
    fn extract_method_arg(&self, _phantom: PhantomData<Arg>) -> Self::HttpMethod;
}
