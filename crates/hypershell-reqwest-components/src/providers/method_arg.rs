use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::{
    HasHttpMethodType, MethodArgExtractor, MethodArgExtractorComponent,
};
use hypershell_components::dsl::{DeleteMethod, GetMethod, PostMethod, PutMethod};
use reqwest::Method;

/// Maps an HTTP method marker to the matching `reqwest::Method` constant. See
/// [the abstract syntax page](https://contextgeneric.dev/docs/projects/hypershell/architecture/abstract-syntax).
pub struct ExtractReqwestMethod;

#[cgp_impl(ExtractReqwestMethod)]
impl<Context> MethodArgExtractor<GetMethod> for Context
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<GetMethod>) -> Method {
        Method::GET
    }
}

#[cgp_impl(ExtractReqwestMethod)]
impl<Context> MethodArgExtractor<PostMethod> for Context
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<PostMethod>) -> Method {
        Method::POST
    }
}

#[cgp_impl(ExtractReqwestMethod)]
impl<Context> MethodArgExtractor<PutMethod> for Context
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<PutMethod>) -> Method {
        Method::PUT
    }
}

#[cgp_impl(ExtractReqwestMethod)]
impl<Context> MethodArgExtractor<DeleteMethod> for Context
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<DeleteMethod>) -> Method {
        Method::DELETE
    }
}
