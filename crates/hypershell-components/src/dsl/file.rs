use core::marker::PhantomData;

pub struct ReadFile<Path>(pub PhantomData<Path>);
