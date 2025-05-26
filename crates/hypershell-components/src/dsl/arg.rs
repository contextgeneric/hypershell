use core::marker::PhantomData;

pub struct StaticArg<Arg>(pub PhantomData<Arg>);

pub struct FieldArg<Tag>(pub PhantomData<Tag>);
