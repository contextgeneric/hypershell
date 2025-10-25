use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::{
    HasHttpMethodType, MethodArgExtractor, MethodArgExtractorComponent,
};
use hypershell_components::dsl::{DeleteMethod, GetMethod, PostMethod, PutMethod};
use reqwest::Method;

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
