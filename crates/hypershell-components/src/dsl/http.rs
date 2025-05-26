use core::marker::PhantomData;

pub struct HttpGet<Url, Params>(pub PhantomData<(Url, Params)>);

pub struct HttpPost<Url, Params>(pub PhantomData<(Url, Params)>);

pub struct WithHeaders<Headers>(pub PhantomData<Headers>);

pub struct Header<Key, Value>(pub PhantomData<(Key, Value)>);
