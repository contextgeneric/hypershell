use core::marker::PhantomData;

pub struct SimpleHttpRequest<Method, Url, Params>(pub PhantomData<(Method, Url, Params)>);

pub struct GetMethod;

pub struct PostMethod;

pub struct WithHeaders<Headers>(pub PhantomData<Headers>);

pub struct Header<Key, Value>(pub PhantomData<(Key, Value)>);

pub struct UrlEncodeArg<Arg>(pub PhantomData<Arg>);
