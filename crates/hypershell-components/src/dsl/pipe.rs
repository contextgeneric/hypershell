use core::marker::PhantomData;

pub struct Pipe<Handlers>(pub PhantomData<Handlers>);
