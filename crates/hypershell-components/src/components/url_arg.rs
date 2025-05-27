use core::marker::PhantomData;

use cgp::prelude::*;

#[cgp_type]
pub trait HasUrlType {
    type Url;
}

#[cgp_component(UrlArgExtractor)]
pub trait CanExtractUrlArg<Arg>: HasUrlType + HasErrorType {
    fn extract_url_arg(&self, _phantom: PhantomData<Arg>) -> Result<Self::Url, Self::Error>;
}
