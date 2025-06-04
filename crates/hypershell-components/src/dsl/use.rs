use core::marker::PhantomData;

pub struct Use<Provider, Code = ()>(pub PhantomData<(Provider, Code)>);
