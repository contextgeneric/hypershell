use core::marker::PhantomData;

pub struct Checksum<Hasher>(pub PhantomData<Hasher>);
