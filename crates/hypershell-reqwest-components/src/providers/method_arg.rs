use core::marker::PhantomData;

use cgp::prelude::*;
use hypershell_components::components::{
    HasHttpMethodType, MethodArgExtractor, MethodArgExtractorComponent,
};
use hypershell_components::dsl::{DeleteMethod, GetMethod, PostMethod, PutMethod};
use reqwest::Method;

pub struct ExtractReqwestMethod;

#[cgp_provider]
impl<Context> MethodArgExtractor<Context, GetMethod> for ExtractReqwestMethod
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<GetMethod>) -> Method {
        Method::GET
    }
}

#[cgp_provider]
impl<Context> MethodArgExtractor<Context, PostMethod> for ExtractReqwestMethod
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<PostMethod>) -> Method {
        Method::POST
    }
}

#[cgp_provider]
impl<Context> MethodArgExtractor<Context, PutMethod> for ExtractReqwestMethod
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<PutMethod>) -> Method {
        Method::PUT
    }
}

#[cgp_provider]
impl<Context> MethodArgExtractor<Context, DeleteMethod> for ExtractReqwestMethod
where
    Context: HasHttpMethodType<HttpMethod = Method>,
{
    fn extract_method_arg(_context: &Context, _phantom: PhantomData<DeleteMethod>) -> Method {
        Method::DELETE
    }
}
