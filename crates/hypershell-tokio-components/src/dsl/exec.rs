use core::marker::PhantomData;

pub struct CoreExec<Path, Args>(pub PhantomData<(Path, Args)>);
