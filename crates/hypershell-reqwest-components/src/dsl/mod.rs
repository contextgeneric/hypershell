use core::marker::PhantomData;

pub struct CoreHttpRequest<Method, Url, Args>(pub PhantomData<(Method, Url, Args)>);
