use core::marker::PhantomData;

pub struct DecodeJson<T>(pub PhantomData<T>);

pub struct EncodeJson;
