use core::marker::PhantomData;

pub struct SimpleExec<Path, Args>(pub PhantomData<(Path, Args)>);

pub struct WithArgs<Args>(pub PhantomData<Args>);

pub struct StaticArg<Arg>(pub PhantomData<Arg>);

pub struct FieldArg<Tag>(pub PhantomData<Tag>);

pub struct JoinArgs<Args>(pub PhantomData<Args>);
