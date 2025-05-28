use core::marker::PhantomData;

pub struct SimpleExec<Path, Args>(pub PhantomData<(Path, Args)>);

pub struct StreamingExec<Path, Args>(pub PhantomData<(Path, Args)>);
