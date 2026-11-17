use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;

/// A context's abstract URL type.
#[cgp_type]
#[prefix(@hypershell.core in DefaultNamespace)]
pub trait HasUrlType {
    type Url;
}

/// Extracts a URL from the context.
#[cgp_component(UrlArgExtractor)]
#[prefix(@hypershell.core in DefaultNamespace)]
#[derive_delegate(UseDelegate<Arg>)]
pub trait CanExtractUrlArg<Arg>: HasUrlType + HasErrorType {
    fn extract_url_arg(&self, _phantom: PhantomData<Arg>) -> Result<Self::Url, Self::Error>;
}
