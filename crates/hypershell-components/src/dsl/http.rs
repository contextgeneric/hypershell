use core::marker::PhantomData;

pub struct HttpGet<Url, Args>(pub PhantomData<(Url, Args)>);
