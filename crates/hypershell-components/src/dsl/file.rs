use core::marker::PhantomData;

pub struct ReadFile<Path>(pub PhantomData<Path>);

pub struct WriteFile<Path>(pub PhantomData<Path>);
