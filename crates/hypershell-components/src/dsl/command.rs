use core::marker::PhantomData;

pub struct SimpleExec<Path, Args>(pub PhantomData<(Path, Args)>);

pub struct StaticArg<Arg>(pub PhantomData<Arg>);

pub struct Join<ArgA, ArgB>(pub PhantomData<(ArgA, ArgB)>);
